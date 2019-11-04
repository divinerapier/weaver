use std::io;
use std::sync::mpsc;

mod directory_error;
mod index_error;
mod needle_error;
mod volume_error;

pub use directory_error::DirectoryError;
pub use index_error::IndexError;
pub use needle_error::NeedleError;
pub use volume_error::VolumeError;

pub enum Error {
    IO(io::Error),
    Channel(String),
    DataCorruption {
        key: String,
        cause: String,
    },
    NotFound(String),
    OpenVolume,
    Path(String),
    Parse {
        from: String,
        to: String,
        cause: String,
    },
    SerdeJSON(serde_json::error::Error),
    Naive(String),
    FileExists(String),
    Volume(VolumeError),
    Directory(DirectoryError),
    Index(IndexError),
    Needle(NeedleError),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            IO(e) => write!(f, "IO. {}", e),
            Channel(e) => write!(f, "Channel. {}", e),
            DataCorruption { key, cause } => {
                write!(f, "DataCorruption. key: {}, cause: {}", key, cause)
            }
            NotFound(e) => write!(f, "NotFound. {}", e),
            OpenVolume => write!(f, "OpenVolume"),
            Path(e) => write!(f, "Path. {}", e),
            Parse { from, to, cause } => {
                write!(f, "Parse. from: {}  to: {}  cause: {}", from, to, cause)
            }
            SerdeJSON(e) => write!(f, "JSON. {}", e),
            Naive(e) => write!(f, "Error. {}", e),
            FileExists(e) => write!(f, "File exists. {}", e),
            Volume(e) => write!(f, "Volume. {}", e),
            Directory(e) => write!(f, "Directory. {}", e),
            Index(e) => write!(f, "Index. {}", e),
            Needle(e) => write!(f, "Needle. {}", e),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            IO(e) => write!(f, "IO. {}", e),
            Channel(e) => write!(f, "Channel. {}", e),
            DataCorruption { key, cause } => {
                write!(f, "DataCorruption. key: {}, cause: {}", key, cause)
            }
            NotFound(e) => write!(f, "NotFound. {}", e),
            OpenVolume => write!(f, "OpenVolume"),
            Path(e) => write!(f, "Path. {}", e),
            Parse { from, to, cause } => {
                write!(f, "Parse. from: {}  to: {}  cause: {}", from, to, cause)
            }
            SerdeJSON(e) => write!(f, "JSON. {}", e),
            Naive(e) => write!(f, "Error. {}", e),
            FileExists(e) => write!(f, "File exists. {}", e),
            Volume(e) => write!(f, "Volume. {}", e),
            Directory(e) => write!(f, "Directory. {}", e),
            Index(e) => write!(f, "Index. {}", e),
            Needle(e) => write!(f, "Needle. {}", e),
        }
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl Error {
    // constructor
    pub fn io(e: io::Error) -> Box<Error> {
        Error::IO(e).into()
    }

    pub fn channel<C>(c: C) -> Box<Error>
    where
        C: Into<String>,
    {
        Error::Channel(c.into()).into()
    }

    pub fn data_corruption<S, Q>(key: S, cause: Q) -> Box<Error>
    where
        S: Into<String>,
        Q: Into<String>,
    {
        let key = key.into();
        let cause = cause.into();
        Error::DataCorruption { key, cause }.into()
    }

    pub fn not_found<S>(cause: S) -> Box<Error>
    where
        S: Into<String>,
    {
        Error::NotFound(cause.into()).into()
    }

    pub fn path<S>(cause: S) -> Box<Error>
    where
        S: Into<String>,
    {
        Error::Path(cause.into()).into()
    }

    pub fn parse<F, T, C>(from: F, to: T, cause: C) -> Box<Error>
    where
        F: Into<String>,
        T: Into<String>,
        C: Into<String>,
    {
        let from = from.into();
        let to = to.into();
        let cause = cause.into();
        Error::Parse { from, to, cause }.into()
    }

    pub fn naive<S>(s: S) -> Box<Error>
    where
        S: Into<String>,
    {
        Error::Naive(s.into()).into()
    }

    pub fn volume(ve: VolumeError) -> Box<Error> {
        Error::Volume(ve).into()
    }

    pub fn directory(de: DirectoryError) -> Box<Error> {
        Error::Directory(de).into()
    }

    pub fn index(ie: IndexError) -> Box<Error> {
        Error::Index(ie).into()
    }
}
// Box the error in case of large data structure when there is no error.
pub type Result<T> = std::result::Result<T, Box<Error>>;

impl From<io::Error> for Error {
    fn from(e: io::Error) -> Self {
        Error::IO(e)
    }
}

impl From<io::Error> for Box<Error> {
    fn from(e: io::Error) -> Self {
        Error::IO(e).into()
    }
}

impl From<std::num::ParseIntError> for std::boxed::Box<Error> {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::parse("string", "int", format!("{:?}", e)).into()
    }
}

impl From<serde_json::error::Error> for std::boxed::Box<Error> {
    fn from(e: serde_json::error::Error) -> Self {
        Error::SerdeJSON(e).into()
    }
}

impl From<String> for Box<Error> {
    fn from(e: String) -> Self {
        Error::Naive(e).into()
    }
}
impl From<mpsc::SendError<Result<bytes::Bytes>>> for Box<Error> {
    fn from(e: mpsc::SendError<Result<bytes::Bytes>>) -> Self {
        Error::Channel(format!("send bytes. {:?}", e)).into()
    }
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        tonic::Status::new(tonic::Code::Unknown, e.to_string())
    }
}
