use std::{path::PathBuf, sync::mpsc};

use thiserror::Error;

pub use directory_error::DirectoryError;
pub use master_error::MasterError;
pub use storage_error::StorageError;

mod directory_error;
mod master_error;
mod storage_error;

pub enum Error {
    Master(String),
    Storage(String),
    Directory(String),
    Internal(String),
    External(Box<dyn std::error::Error + Send>),
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
        E: std::error::Error + Send + 'static,
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

#[derive(Debug, Error)]
pub enum VolumeError {
    #[error("volume: {0}/{1} not found")]
    NeedleNotFound(u64, u64),

    #[error("io")]
    IO(#[from] std::io::Error),

    #[error("channel")]
    ChannelClosed,

    #[error("not a dir {0}")]
    NotDir(PathBuf),
}

impl PartialEq for VolumeError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::NeedleNotFound(ref l0, ref l1), Self::NeedleNotFound(r0, r1)) => {
                l0 == r0 && l1 == r1
            }
            (Self::IO(l0), Self::IO(r0)) => l0.kind() == r0.kind(),
            (_, _) => false,
        }
    }
}

impl Eq for VolumeError {}
