use std::io::Read;
use std::sync::Arc;

use futures::future::Future;
use grpcio::{Environment, Service};

pub struct Server {
    service: Service,
}

impl Server {
    pub fn new(service: Service) -> Server {
        Server { service }
    }
    pub fn serve<H: Into<String>>(self, host: H, port: u16) {
        let mut server = grpcio::ServerBuilder::new(Arc::new(Environment::new(1)))
            .register_service(self.service)
            .bind(host, port)
            .build()
            .unwrap();
        server.start();
        let (tx, rx) = futures::sync::oneshot::channel();
        std::thread::spawn(move || {
            log::info!("Press ENTER to exit...");
            let _ = std::io::stdin().read(&mut [0]).unwrap();
            tx.send(())
        });
        let _ = rx.wait();
        let _ = server.shutdown().wait();
    }
}
