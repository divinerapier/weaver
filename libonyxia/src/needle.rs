use std::sync::mpsc::Receiver;

use bytes::Bytes;

use crate::error::Result;

#[allow(dead_code)]
pub struct Needle {
    pub body: NeedleBody,
    pub length: usize,
}

pub enum NeedleBody {
    SinglePart(Bytes),
    MultiParts(Receiver<Result<Bytes>>),
}
