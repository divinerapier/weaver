#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => {
        crate::error::Error::normal(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! external_error {
    ($arg:tt) => {
        crate::error::Error::dependency($arg)
    };
}

#[macro_export]
macro_rules! master_error {
    ($($arg:tt)*) => {
        crate::error::Error::master(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! directory_error {
    ($($arg:tt)*) => {
        crate::error::Error::directory(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! storage_error {
    ($($arg:tt)*) => {
        crate::error::Error::storage(format!($($arg)*))
    };
}
