#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Index {
    pub path: String,
    pub offset: usize,
    pub length: usize,
}

impl Index {
    pub fn new(path: String, offset: usize, length: usize) -> Index {
        Index {
            path,
            offset,
            length,
        }
    }
}

pub struct RawIndex {
    pub offset: usize,
    pub length: usize,
}

impl RawIndex {
    pub fn new(offset: usize, length: usize) -> RawIndex {
        RawIndex { offset, length }
    }
}
