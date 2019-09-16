#[macro_export]
macro_rules! boxed_naive {
    ($($arg:tt)*) => {
        crate::error::Error::naive(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! naive {
    ($($arg:tt)*) => {
        crate::error::Error::Naive(format!($($arg)*))
    };
}

#[macro_export]
macro_rules! boxed_volume_not_found {
    ($arg:tt) => {
        crate::error::Error::volume(crate::error::VolumeError::not_found($arg))
    };
}

#[macro_export]
macro_rules! volume_not_found {
    ($arg:tt) => {
        crate::error::Error::Volume(crate::error::VolumeError::not_found($arg))
    };
}

#[macro_export]
macro_rules! boxed_no_writable_volumes {
    () => {
        crate::error::Error::directory(crate::error::DirectoryError::GetWritableVolume)
    };
}

#[macro_export]
macro_rules! no_writable_volumes {
    () => {
        crate::error::Error::Directory(crate::error::DirectoryError::GetWritableVolume)
    };
}

#[macro_export]
macro_rules! boxed_volume_create {
    ($arg:tt, $cause:tt) => {
        crate::error::Error::volume(crate::error::VolumeError::create($arg, $cause))
    };
}

#[macro_export]
macro_rules! boxed_index_create {
    ($arg:tt, $cause:tt) => {
        crate::error::Error::index(crate::error::IndexError::create($arg, $cause))
    };
}
