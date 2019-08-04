use failure::Fail;

#[derive(Debug, Fail)]
pub enum IndexError {
    #[fail(display = "Create. index: {}, cause: {}", index, cause)]
    Create { index: usize, cause: String },
}

impl IndexError {
    pub fn create<C>(index: usize, cause: C) -> IndexError
    where
        C: Into<String>,
    {
        let cause = cause.into();
        IndexError::Create { index, cause }.into()
    }
}
