use std::fmt::{Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub enum MasterError {}

impl Display for MasterError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        write!(f, "")
    }
}
