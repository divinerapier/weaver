use std::cell::Cell;
use std::fs;
use std::io::Write;

use actix_multipart::{Field, Multipart, MultipartError};
use actix_web::{error, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::future::{err, Either};
use futures::{Future, Stream};

pub struct AppState {
    pub counter: Cell<usize>,
}

pub fn save_file(field: Field, filepath: &str) -> impl Future<Item = i64, Error = Error> {
    let current_dir = std::env::current_dir().unwrap();
    let file_path_string = current_dir.join(filepath);
    log::info!("upload file to path: {:?}", file_path_string);
    let parent = file_path_string.parent().unwrap();
    std::fs::create_dir_all(parent).unwrap();
    let file = match fs::File::create(file_path_string) {
        Ok(file) => file,
        Err(e) => return Either::A(err(error::ErrorInternalServerError(e))),
    };
    Either::B(
        field
            .fold((file, 0i64), move |(mut file, mut acc), bytes| {
                // fs operations are blocking, we have to execute writes
                // on threadpool
                web::block(move || {
                    file.write_all(bytes.as_ref()).map_err(|e| {
                        log::info!("file.write_all failed: {:?}", e);
                        MultipartError::Payload(error::PayloadError::Io(e))
                    })?;
                    acc += bytes.len() as i64;
                    Ok((file, acc))
                })
                .map_err(|e: error::BlockingError<MultipartError>| match e {
                    error::BlockingError::Error(e) => e,
                    error::BlockingError::Canceled => MultipartError::Incomplete,
                })
            })
            .map(|(_, acc)| acc)
            .map_err(|e| {
                log::error!("save_file failed, {:?}", e);
                error::ErrorInternalServerError(e)
            }),
    )
}

pub fn upload(
    req: HttpRequest,
    multipart: Multipart,
    counter: web::Data<Cell<usize>>,
) -> impl Future<Item = HttpResponse, Error = Error> {
    counter.set(counter.get() + 1);
    let path = req.match_info().get("path").unwrap().to_owned();
    log::info!("path: {}", path);

    multipart
        .map_err(error::ErrorInternalServerError)
        .map(move |field| save_file(field, &path).into_stream())
        .flatten()
        .collect()
        .map(|sizes| HttpResponse::Ok().json(sizes))
        .map_err(|e| {
            log::error!("multipart upload: {}", e);
            e
        })
}

pub fn download(req: HttpRequest) -> HttpResponse {
    let path = req.match_info().get("path").unwrap().to_owned();

    match std::fs::File::open(path) {
        Err(e) => match e.kind() {
            std::io::ErrorKind::NotFound => HttpResponse::NotFound().body("file not found"),
            err @ _ => {
                let message = format!("{:?}", err);
                HttpResponse::InternalServerError().body(message)
            }
        },
        Ok(f) => HttpResponse::Ok().streaming(libonyxia::file::FileStream::from_std_file(f)),
    }
}

fn index() -> HttpResponse {
    let html = r#"<html>
        <head><title>Upload Test</title>
            <script type="text/javascript">
                function upload(input) {
                    var filename = document.getElementById('FileName').value;
                    console.log('filename = ' + filename);
                    document.getElementById('UploadForm').action = 'api/' + filename
                }
            </script>
        </head>
        <body>
            <form target="_blank" method="post" enctype="multipart/form-data" id="UploadForm" onsubmit="return upload(this);">
                <input type="input" name="name" id="FileName"></input>
                <input type="file" name="file" id="FileContent"></input>
                <input type="submit" value="Submit" ></button>
            </form>
        </body>
    </html>"#;

    HttpResponse::Ok().body(html)
}

fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::from_env(env_logger::Env::default().default_filter_or("trace")).init();
    log::set_max_level(log::LevelFilter::max());

    HttpServer::new(|| {
        App::new()
            .data(Cell::new(0usize))
            .wrap(middleware::Logger::default())
            .service(
                // https://actix.rs/docs/url-dispatch/
                web::resource("/api/{path:.*}")
                    .route(web::get().to(download))
                    .route(web::post().to_async(upload)),
            )
            .service(web::resource("/index").route(web::get().to(index)))
    })
    .bind("127.0.0.1:18080")?
    .run()
}
