use failure::Fail;

#[derive(Debug, Fail)]
pub enum IndexError {
    #[fail(display = "Create. index: {}, cause: {}", index, cause)]
    Create { index: u32, cause: String },
}

impl IndexError {
    pub fn create<C>(index: u32, cause: C) -> IndexError
    where
        C: Into<String>,
    {
        let cause = cause.into();
        IndexError::Create { index, cause }.into()
    }
}
