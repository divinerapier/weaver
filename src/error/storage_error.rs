use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum StorageError {}

impl Display for StorageError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "")
    }
}

// use failure::Fail;

// #[derive(Debug, Fail)]
// pub enum StorageError {
//     #[fail(display = "Create. id: {}, cause: {}", id, cause)]
//     Create { id: u32, cause: String },

//     #[fail(display = "ReadOnly. id: {}", _0)]
//     ReadOnly(u32),

//     #[fail(
//         display = "Overflow. id: {}, max: {}, current: {}, todo: {}",
//         id, max, current, todo
//     )]
//     Overflow {
//         id: u32,
//         max: u64,
//         current: u64,
//         todo: u64,
//     },

//     #[fail(display = "Data corruption. id: {}, cause: {}", id, cause)]
//     DataCorruption { id: u32, cause: String },

//     #[fail(
//         display = "WriteLengthMismatch. id: {}, path: {}, input length: {}, receive length: {}",
//         id, path, input_length, receive_length
//     )]
//     WriteLengthMismatch {
//         id: u32,
//         path: String,
//         input_length: usize,
//         receive_length: usize,
//     },
//     #[fail(display = "Not Found. {}", _0)]
//     NotFound(u32),

//     #[fail(display = "Write Needle. path: {}, cause: {}", path, cause)]
//     WriteNeedle { path: String, cause: String },
// }

// impl StorageError {
//     pub fn create<C>(id: u32, cause: C) -> StorageError
//     where
//         C: Into<String>,
//     {
//         let cause = cause.into();
//         StorageError::Create { id, cause }
//     }

//     pub fn readonly(index: u32) -> StorageError {
//         StorageError::ReadOnly(index)
//     }

//     pub fn overflow(id: u32, max: u64, current: u64, todo: u64) -> StorageError {
//         StorageError::Overflow {
//             id,
//             max,
//             current,
//             todo,
//         }
//     }

//     pub fn data_corruption<C>(id: u32, cause: C) -> StorageError
//     where
//         C: Into<String>,
//     {
//         let cause = cause.into();
//         StorageError::DataCorruption { id, cause }
//     }

//     pub fn write_length_mismatch<P>(
//         id: u32,
//         path: P,
//         input_length: usize,
//         receive_length: usize,
//     ) -> StorageError
//     where
//         P: Into<String>,
//     {
//         let path = path.into();
//         StorageError::WriteLengthMismatch {
//             id,
//             path,
//             input_length,
//             receive_length,
//         }
//     }

//     pub fn not_found(id: u32) -> StorageError {
//         StorageError::NotFound(id)
//     }

//     pub fn write_needle<P, C>(path: P, cause: C) -> StorageError
//     where
//         P: Into<String>,
//         C: Into<String>,
//     {
//         let path = path.into();
//         let cause = cause.into();
//         StorageError::WriteNeedle { path, cause }
//     }
// }
