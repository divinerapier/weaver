use failure::Fail;

#[derive(Debug, Fail)]
pub enum DirectoryError {
    #[fail(display = "Create. id: {}, cause: {}", id, cause)]
    Create { id: usize, cause: String },

    #[fail(display = "ReadOnly. id: {}", _0)]
    ReadOnly(usize),

    #[fail(
        display = "Overflow. id: {}, max: {}, current: {}, todo: {}",
        id, max, current, todo
    )]
    Overflow {
        id: usize,
        max: u64,
        current: u64,
        todo: u64,
    },

    #[fail(display = "Data corruption. id: {}, cause: {}", id, cause)]
    DataCorruption { id: usize, cause: String },

    #[fail(
        display = "WriteLengthMismatch. id: {}, path: {}, input length: {}, receive length: {}",
        id, path, input_length, receive_length
    )]
    WriteLengthMismatch {
        id: usize,
        path: String,
        input_length: usize,
        receive_length: usize,
    },
    #[fail(display = "Not Found. {}", _0)]
    NotFound(usize),

    #[fail(display = "Write Needle. path: {}, cause: {}", path, cause)]
    WriteNeedle { path: String, cause: String },

    #[fail(display = "GetWritableVolume")]
    GetWritableVolume,
}
