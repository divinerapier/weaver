#![feature(try_trait)]

#[macro_use]
pub mod macros;

pub mod directory;
pub mod error;
pub mod file;
pub mod index;
pub mod needle;
pub mod server;
pub mod store;
pub mod utils;

#[cfg(test)]
mod test {
    #[test]
    fn test_macro_naive() {
        match naive!("data: {}, {}, {}, {}, {}", 0, 1, 2, 3, 4) {
            crate::error::Error::Naive(s) => {
                log::debug!("naive error message: {}", s);
                assert_eq!(s, "data: 0, 1, 2, 3, 4");
            }
            _ => unreachable!(),
        }
        match *boxed_naive!("data: {}, {}, {}, {}, {}", 0, 1, 2, 3, 4) {
            crate::error::Error::Naive(s) => {
                log::debug!("naive error message: {}", s);
                assert_eq!(s, "data: 0, 1, 2, 3, 4");
            }
            _ => unreachable!(),
        }
    }
    #[test]
    fn test_macro_volume_not_found() {
        match volume_not_found!(3) {
            crate::error::Error::Volume(volume_error) => match volume_error {
                crate::error::VolumeError::NotFound(n) => {
                    assert_eq!(n, 3);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }

        match *boxed_volume_not_found!(3) {
            crate::error::Error::Volume(volume_error) => match volume_error {
                crate::error::VolumeError::NotFound(n) => {
                    assert_eq!(n, 3);
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
}
