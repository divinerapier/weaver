use std::sync::mpsc;

mod directory_error;
mod master_error;
mod storage_error;

pub use directory_error::DirectoryError;
pub use master_error::MasterError;
pub use storage_error::StorageError;

pub enum Error {
    Master(String),
    Storage(String),
    Directory(String),
    Internal(String),
    External(Box<dyn std::error::Error>),
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Master(e) => write!(f, "Master. {}", e),
            Storage(e) => write!(f, "Volume. {}", e),
            Directory(e) => write!(f, "Directory. {}", e),
            Internal(e) => write!(f, "{}", e),
            External(e) => write!(f, "{}", e),
        }
    }
}

impl std::fmt::Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match self {
            Master(e) => write!(f, "Master. {}", e),
            Storage(e) => write!(f, "Volume. {}", e),
            Directory(e) => write!(f, "Directory. {}", e),
            Internal(e) => write!(f, "{}", e),
            External(e) => write!(f, "{}", e),
        }
    }
}

unsafe impl Send for Error {}
unsafe impl Sync for Error {}

impl std::error::Error for Error {}

impl Error {
    pub fn master<S>(s: S) -> Error
    where
        S: Into<String>,
    {
        Error::Master(s.into())
    }

    pub fn storage<S>(s: S) -> Error
    where
        S: Into<String>,
    {
        Error::Storage(s.into())
    }

    pub fn directory<S>(s: S) -> Error
    where
        S: Into<String>,
    {
        Error::Directory(s.into())
    }

    pub fn normal<S>(s: S) -> Error
    where
        S: Into<String>,
    {
        Error::Internal(s.into())
    }

    pub fn dependency<E>(e: E) -> Error
    where
        E: std::error::Error + 'static,
    {
        Error::External(Box::new(e))
    }
}

// Box the error in case of large data structure when there is no error.
pub type Result<T> = std::result::Result<T, Error>;

impl From<std::io::Error> for Error {
    fn from(e: std::io::Error) -> Self {
        Error::External(Box::new(e))
    }
}

impl From<std::num::ParseIntError> for Error {
    fn from(e: std::num::ParseIntError) -> Self {
        Error::External(Box::new(e))
    }
}

impl From<serde_json::error::Error> for Error {
    fn from(e: serde_json::error::Error) -> Self {
        Error::External(Box::new(e))
    }
}

impl From<mpsc::SendError<Result<bytes::Bytes>>> for Error {
    fn from(e: mpsc::SendError<Result<bytes::Bytes>>) -> Self {
        Error::External(Box::new(e))
    }
}

impl From<Error> for tonic::Status {
    fn from(e: Error) -> Self {
        tonic::Status::new(tonic::Code::Unknown, e.to_string())
    }
}
