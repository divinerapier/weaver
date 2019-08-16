#[allow(dead_code)]
#[derive(serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Index {
    pub path: String,
    pub volume_id: usize,
    pub offset: usize,
    pub length: usize,
}

impl Index {
    pub fn new(path: String, volume_id: usize, offset: usize, length: usize) -> Index {
        Index {
            path,
            volume_id,
            offset,
            length,
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RawIndex {
    pub volume_id: usize,
    pub offset: usize,
    pub length: usize,
}

impl RawIndex {
    pub fn new(volume_id: usize, offset: usize, length: usize) -> RawIndex {
        RawIndex { volume_id,offset, length }
    }
}

impl Default for RawIndex {
    fn default() -> RawIndex {
        RawIndex {
            volume_id:0,
            offset: 0,
            length: 0,
        }
    }
}
