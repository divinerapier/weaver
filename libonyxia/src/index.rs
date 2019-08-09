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

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RawIndex {
    pub offset: usize,
    pub length: usize,
}

impl RawIndex {
    pub fn new(offset: usize, length: usize) -> RawIndex {
        RawIndex { offset, length }
    }
}

impl Default for RawIndex {
    fn default() -> RawIndex {
        RawIndex {
            offset: 0,
            length: 0,
        }
    }
}
