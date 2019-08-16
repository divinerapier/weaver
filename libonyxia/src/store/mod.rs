pub mod volume;

/// Store consists of many volumes.
pub struct Store {
    // volumes
    volumes: Vec<volume::Volume>,
}
