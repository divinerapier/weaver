#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Status {
    #[prost(enumeration = "Code", tag = "1")]
    pub code: i32,
    #[prost(string, tag = "2")]
    pub message: std::string::String,
}
/// Needle is the unit of a volume. Align to 8 bytes.
///    | 00 01 02 03 04 05 06 07 08 09 10 11 12 13 14 15 |
/// 00 |header_length|body_length| header                |
/// 01 | header        | padding = 8 - header_length % 8 |
/// 02 | body            | padding = 8 - body_length % 8 |
///
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Needle {
    /// let hl = headler_length;
    /// let bl = body_length;
    ///
    /// fn pad(i: u32) -> u32 {
    ///   match i & 7 {
    ///     0 => i,
    ///     d @ _ => i + 8 - d,
    ///   }
    /// }
    ///
    /// let header_padded = pad(header_length);
    /// let body_padded = pad(body_length);
    /// let data = read(reader, header_padded + body_padded);
    /// let header = &data[0..header_length];
    /// let body = &data[header_padded..header_padded+body_length];
    #[prost(message, optional, tag = "3")]
    pub header: ::std::option::Option<NeedleHeader>,
    #[prost(bytes, tag = "4")]
    pub body: std::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NeedleHeader {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub cookie: u64,
    #[prost(uint64, tag = "3")]
    pub offset: u64,
    #[prost(uint32, tag = "4")]
    pub size: u32,
    #[prost(uint64, tag = "5")]
    pub total_size: u64,
    #[prost(uint32, tag = "6")]
    pub crc: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReplicaReplacement {
    #[prost(uint32, tag = "1")]
    pub data_center_count: u32,
    /// in same data center
    #[prost(uint32, tag = "2")]
    pub rack_count: u32,
    /// in same rack
    #[prost(uint32, tag = "3")]
    pub node_count: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Attribute {
    #[prost(uint64, tag = "1")]
    pub size: u64,
    /// unix time in seconds
    #[prost(int64, tag = "3")]
    pub crtime: i64,
    #[prost(string, tag = "4")]
    pub mime: std::string::String,
    #[prost(enumeration = "FileMode", tag = "5")]
    pub mode: i32,
    #[prost(uint32, tag = "6")]
    pub uid: u32,
    #[prost(uint32, tag = "7")]
    pub gid: u32,
    #[prost(message, optional, tag = "8")]
    pub replica_replacement: ::std::option::Option<ReplicaReplacement>,
    #[prost(int32, tag = "9")]
    pub ttl_sec: i32,
}
/// Chunk refers to a needle.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Chunk {
    #[prost(uint32, tag = "1")]
    pub volume_id: u32,
    #[prost(uint64, tag = "2")]
    pub needle_id: u64,
    #[prost(uint64, tag = "3")]
    pub cookie: u64,
    #[prost(int64, tag = "4")]
    pub offset: i64,
    #[prost(uint64, tag = "5")]
    pub size: u64,
    #[prost(string, tag = "6")]
    pub etag: std::string::String,
}
/// Entry indicates an object info, includes name, chunks and attibute,
/// used by directory.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Entry {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
    #[prost(message, repeated, tag = "3")]
    pub chunks: ::std::vec::Vec<Chunk>,
    #[prost(message, optional, tag = "4")]
    pub attribute: ::std::option::Option<Attribute>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct FullEntry {
    #[prost(string, tag = "1")]
    pub dir: std::string::String,
    #[prost(message, optional, tag = "2")]
    pub entry: ::std::option::Option<Entry>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Locations {
    #[prost(message, repeated, tag = "1")]
    pub locations: ::std::vec::Vec<Location>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Location {
    #[prost(string, tag = "1")]
    pub url: std::string::String,
    #[prost(string, tag = "2")]
    pub presigned_url: std::string::String,
}
/// Node is a machine.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Node {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub volume_count: u64,
    #[prost(uint64, tag = "3")]
    pub max_volume_count: u64,
    #[prost(uint64, tag = "4")]
    pub free_volume_count: u64,
    #[prost(uint64, tag = "5")]
    pub active_volume_count: u64,
    #[prost(message, repeated, tag = "6")]
    pub volume_infos: ::std::vec::Vec<VolumeInformationMessage>,
}
/// Rack consists of many nodes.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Rack {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub volume_count: u64,
    #[prost(uint64, tag = "3")]
    pub max_volume_count: u64,
    #[prost(uint64, tag = "4")]
    pub free_volume_count: u64,
    #[prost(uint64, tag = "5")]
    pub active_volume_count: u64,
    #[prost(message, repeated, tag = "6")]
    pub nodes: ::std::vec::Vec<Node>,
}
/// DataCenter consists of many racks.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DataCenter {
    #[prost(uint64, tag = "1")]
    pub id: u64,
    #[prost(uint64, tag = "2")]
    pub volume_count: u64,
    #[prost(uint64, tag = "3")]
    pub max_volume_count: u64,
    #[prost(uint64, tag = "4")]
    pub free_volume_count: u64,
    #[prost(uint64, tag = "5")]
    pub active_volume_count: u64,
    #[prost(message, repeated, tag = "6")]
    pub racks: ::std::vec::Vec<Rack>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeInformationMessage {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(uint64, tag = "2")]
    pub size: u64,
    #[prost(string, tag = "3")]
    pub bucket: std::string::String,
    #[prost(uint64, tag = "4")]
    pub file_count: u64,
    #[prost(uint64, tag = "5")]
    pub delete_count: u64,
    #[prost(uint64, tag = "6")]
    pub deleted_byte_count: u64,
    #[prost(bool, tag = "7")]
    pub read_only: bool,
    #[prost(message, optional, tag = "8")]
    pub replica_replacement: ::std::option::Option<ReplicaReplacement>,
    #[prost(uint32, tag = "9")]
    pub version: u32,
    #[prost(uint32, tag = "10")]
    pub ttl: u32,
    #[prost(uint32, tag = "11")]
    pub compact_revision: u32,
    #[prost(int64, tag = "12")]
    pub modified_at_second: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct VolumeShortInformationMessage {
    #[prost(uint32, tag = "1")]
    pub id: u32,
    #[prost(string, tag = "2")]
    pub bucket: std::string::String,
    #[prost(message, optional, tag = "3")]
    pub replica_replacement: ::std::option::Option<ReplicaReplacement>,
    #[prost(uint32, tag = "4")]
    pub version: u32,
    #[prost(uint32, tag = "5")]
    pub ttl: u32,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SuperBlockExtra {
    #[prost(message, optional, tag = "1")]
    pub erasure_coding: ::std::option::Option<super_block_extra::ErasureCoding>,
}
pub mod super_block_extra {
    #[derive(Clone, PartialEq, ::prost::Message)]
    pub struct ErasureCoding {
        #[prost(uint32, tag = "1")]
        pub data: u32,
        #[prost(uint32, tag = "2")]
        pub parity: u32,
        #[prost(uint32, repeated, tag = "3")]
        pub volume_ids: ::std::vec::Vec<u32>,
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StorageType {
    #[prost(message, optional, tag = "1")]
    pub replica_replacement: ::std::option::Option<ReplicaReplacement>,
    #[prost(string, tag = "2")]
    pub ttl: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Bucket {
    #[prost(string, tag = "1")]
    pub name: std::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum FileMode {
    UnknownFileMode = 0,
    Directory = 1,
    RegularFile = 2,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Code {
    UnknownCode = 0,
    Ok = 1,
    Failed = 2,
}
