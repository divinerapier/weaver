use std::io;
use std::sync::mpsc;

use failure::Fail;

mod index_error;
mod volume_error;

pub use index_error::IndexError;
pub use volume_error::VolumeError;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO. {}", _0)]
    IO(#[fail(cause)] io::Error),

    #[fail(display = "Channel. {}", _0)]
    Channel(String),

    #[fail(display = "Data corruption. key: {}, cause: {}", key, cause)]
    DataCorruption { key: String, cause: String },

    #[fail(display = "Not found. {}", _0)]
    NotFound(String),

    #[fail(display = "Open volume")]
    OpenVolume,

    #[fail(display = "Path. {}", _0)]
    Path(String),

    #[fail(display = "Parse. from: {}  to: {}  cause: {}", from, to, cause)]
    Parse {
        from: String,
        to: String,
        cause: String,
    },

    #[fail(display = "JSON. {}", _0)]
    SerdeJSON(serde_json::error::Error),

    #[fail(display = "Error. {}", _0)]
    Naive(String),

    #[fail(display = "File exists. {}", _0)]
    FileExists(String),

    #[fail(display = "Volume {}", _0)]
    Volume(VolumeError),

    #[fail(display = "Index {}", _0)]
    Index(IndexError),
}

impl Error {
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

    pub fn index(ie: IndexError) -> Box<Error> {
        Error::Index(ie).into()
    }
}
// Box the error in case of large data structure when there is no error.
pub type Result<T> = std::result::Result<T, Box<Error>>;

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
