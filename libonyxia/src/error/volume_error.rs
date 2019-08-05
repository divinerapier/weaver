use failure::Fail;

#[derive(Debug, Fail)]
pub enum VolumeError {
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
}

impl VolumeError {
    pub fn create<C>(id: usize, cause: C) -> VolumeError
    where
        C: Into<String>,
    {
        let cause = cause.into();
        VolumeError::Create { id, cause }
    }

    pub fn readonly(index: usize) -> VolumeError {
        VolumeError::ReadOnly(index)
    }

    pub fn overflow(id: usize, max: u64, current: u64, todo: u64) -> VolumeError {
        VolumeError::Overflow {
            id,
            max,
            current,
            todo,
        }
    }

    pub fn write_length_mismatch<P>(
        id: usize,
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
}
