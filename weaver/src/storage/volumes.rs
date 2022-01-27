use std::path::Path;

use super::volume_file::VolumeImpl;
use crate::{error::VolumeError, needle::Needle};

pub struct Volumes {
    volumes: Vec<VolumeImpl>,
}

impl Volumes {
    pub fn new<P: AsRef<Path>>(dir: P, max: usize) {}
}

impl Volumes {
    pub async fn read(
        &self,
        vid: u64,
        nid: u64,
    ) -> std::result::Result<Option<Needle>, VolumeError> {
        if self.volumes.len() >= vid as usize {
            return Ok(None);
        }
        let data = self.volumes[vid as usize].read(nid).await?;
        Ok(None)
    }

    pub async fn write(
        &self,
        vid: u64,
        nid: u64,
        needle: Needle,
    ) -> std::result::Result<Option<()>, VolumeError> {
        if self.volumes.len() >= vid as usize {
            return Ok(None);
        }
        self.volumes[vid as usize].write(nid, vec![]).await?;
        Ok(Some(()))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    fn test() {
        fn foo<S: Send + Sync>() {}

        foo::<Volumes>();
    }
}
