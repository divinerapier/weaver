use failure::Fail;

#[derive(Debug, Fail)]
pub enum VolumeError {
    #[fail(display = "Create. index: {}, cause: {}", index, cause)]
    Create { index: usize, cause: String },
}

impl VolumeError {
    pub fn create<C>(index: usize, cause: C) -> VolumeError where C: Into<String> {
        let cause = cause.into();
        VolumeError::Create{index, cause}.into()
    }
}