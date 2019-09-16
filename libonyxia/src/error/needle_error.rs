use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum NeedleError {
    NotFound(u64),
}

impl Display for NeedleError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        match self {
            NeedleError::NotFound(message) => write!(f, "not found: {}", message),
        }
    }
}

impl std::error::Error for NeedleError {}
