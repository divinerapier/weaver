use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum DirectoryError {}

impl Display for DirectoryError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "")
    }
}
