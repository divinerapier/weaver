#[allow(dead_code)]
#[derive(Copy, Clone, serde::Serialize, serde::Deserialize, Debug, PartialEq)]
pub struct Index {
    pub needle_id: u64,
    pub volume_id: u32,
    pub offset: usize,
    pub length: usize,
}

impl Index {
    pub fn new(needle_id: u64, volume_id: u32, offset: usize, length: usize) -> Index {
        Index {
            needle_id,
            volume_id,
            offset,
            length,
        }
    }
}

#[derive(Copy, Clone, Debug, serde::Serialize)]
pub struct RawIndex {
    pub volume_id: u32,
    pub offset: usize,
    pub length: usize,
}

impl RawIndex {
    pub fn new(volume_id: u32, offset: usize, length: usize) -> RawIndex {
        RawIndex {
            volume_id,
            offset,
            length,
        }
    }
}

impl Default for RawIndex {
    fn default() -> RawIndex {
        RawIndex {
            volume_id: 0,
            offset: 0,
            length: 0,
        }
    }
}
