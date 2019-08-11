#[allow(dead_code)]
#[allow(unknown_lints)]
#[allow(clippy::all)]
#[allow(renamed_and_removed_lints)]
#[allow(bare_trait_objects)]

pub mod directory {
    include!(concat!(env!("OUT_DIR"), "/directory/mod.rs"));

    #[cfg(feature = "prost-codec")]
    pub use self::grpc::directory::*;
}

pub mod volume {
    include!(concat!(env!("OUT_DIR"), "/volume/mod.rs"));

    #[cfg(feature = "prost-codec")]
    pub use self::grpc::volume::*;
}
