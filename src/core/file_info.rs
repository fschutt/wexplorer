//! Struct holding metadata about a certain file without having to have it open.
//! Thin wrapper around the core Rust `fs::metadata`.

pub struct FileInfo {
    pub simple_properties: ::std::fs::Metadata,
}
