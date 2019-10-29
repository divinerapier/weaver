use failure::Fail;

#[derive(Debug, Fail)]
pub enum VolumeError {
    #[fail(display = "Create. id: {}, cause: {}", id, cause)]
    Create { id: u32, cause: String },

    #[fail(display = "ReadOnly. id: {}", _0)]
    ReadOnly(u32),

    #[fail(
        display = "Overflow. id: {}, max: {}, current: {}, todo: {}",
        id, max, current, todo
    )]
    Overflow {
        id: u32,
        max: u64,
        current: u64,
        todo: u64,
    },

    #[fail(display = "Data corruption. id: {}, cause: {}", id, cause)]
    DataCorruption { id: u32, cause: String },

    #[fail(
        display = "WriteLengthMismatch. id: {}, path: {}, input length: {}, receive length: {}",
        id, path, input_length, receive_length
    )]
    WriteLengthMismatch {
        id: u32,
        path: String,
        input_length: usize,
        receive_length: usize,
    },
    #[fail(display = "Not Found. {}", _0)]
    NotFound(u32),

    #[fail(display = "Write Needle. path: {}, cause: {}", path, cause)]
    WriteNeedle { path: String, cause: String },
}

impl VolumeError {
    pub fn create<C>(id: u32, cause: C) -> VolumeError
    where
        C: Into<String>,
    {
        let cause = cause.into();
        VolumeError::Create { id, cause }
    }

    pub fn readonly(index: u32) -> VolumeError {
        VolumeError::ReadOnly(index)
    }

    pub fn overflow(id: u32, max: u64, current: u64, todo: u64) -> VolumeError {
        VolumeError::Overflow {
            id,
            max,
            current,
            todo,
        }
    }

    pub fn data_corruption<C>(id: u32, cause: C) -> VolumeError
    where
        C: Into<String>,
    {
        let cause = cause.into();
        VolumeError::DataCorruption { id, cause }
    }

    pub fn write_length_mismatch<P>(
        id: u32,
        path: P,
        input_length: usize,
        receive_length: usize,
    ) -> VolumeError
    where
        P: Into<String>,
    {
        let path = path.into();
        VolumeError::WriteLengthMismatch {
            id,
            path,
            input_length,
            receive_length,
        }
    }

    pub fn not_found(id: u32) -> VolumeError {
        VolumeError::NotFound(id)
    }

    pub fn write_needle<P, C>(path: P, cause: C) -> VolumeError
    where
        P: Into<String>,
        C: Into<String>,
    {
        let path = path.into();
        let cause = cause.into();
        VolumeError::WriteNeedle { path, cause }
    }
}
