use std::io;

use failure::Fail;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "IO error. {}", _0)]
    IO(#[fail(cause)] io::Error),

    #[fail(display = "Data corruption. key: {}, cause: {}", key, cause)]
    DataCorruption { key: String, cause: String },

    #[fail(display = "Not found corruption. cause: {}", cause)]
    NotFound { cause: String },

    #[fail(display = "Open volume")]
    OpenVolume,

    #[fail(display = "File System. cause: {}", cause)]
    FileSystem { cause: String },

    #[fail(display = "Parse. cause: {}", cause)]
    Parse { cause: String },

    #[fail(display = "Trim")]
    Trim,

    #[fail(display = "ParseIntError. {}", _0)]
    ParseIntError(std::num::ParseIntError),

    #[fail(display = "SerdeJsonError. {}", _0)]
    SerdeJsonError(serde_json::error::Error),

    #[fail(display = "NaiveError. {}", _0)]
    NaiveError(String),
}

impl Error {
    pub fn not_found<S>(cause: S) -> Box<Error>
    where
        S: Into<String>,
    {
        let cause = cause.into();
        Error::NotFound { cause }.into()
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

    pub fn file_system<S>(cause: S) -> Box<Error>
    where
        S: Into<String>,
    {
        let cause = cause.into();
        Error::FileSystem { cause }.into()
    }

    pub fn parse<S>(cause: S) -> Box<Error>
    where
        S: Into<String>,
    {
        let cause = cause.into();
        Error::Parse { cause }.into()
    }

    pub fn trim() -> Box<Error> {
        Error::Trim.into()
    }

    pub fn naive<S>(s: S) -> Box<Error>
    where
        S: Into<String>,
    {
        Error::NaiveError(s.into()).into()
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
        Error::ParseIntError(e).into()
    }
}

impl From<serde_json::error::Error> for std::boxed::Box<Error> {
    fn from(e: serde_json::error::Error) -> Self {
        Error::SerdeJsonError(e).into()
    }
}

impl From<String> for Box<Error> {
    fn from(e: String) -> Self {
        Error::NaiveError(e).into()
    }
}
