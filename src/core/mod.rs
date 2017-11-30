//! Core functionality (search, system info, file listing, history and path handling)

/// The core application, basically a super-struct holding all the necessary information
/// to display a UI. The `App` is then structured into sub-modules.
pub mod app;

/// A configuration, which is given to the `App` at startup time (ex. whether to show
/// the ribbon by default, custom library paths, etc.)
pub mod app_config;
/// File information, thin wrapper around `fs::Metadata`
pub mod file_info;
/// Struct holding the history and last visited paths
pub mod history;
/// Virtual directory, holding multiple `PathBuf` structs as a collection. 
/// For example, the search functionality can create a `VirtualDirectory`, which
/// contains `PathBuf`s to all the found files.
pub mod virtual_directory;
/// Networking-related stuff (connecting to file server and mounting remote drives)
pub mod networking;
/// Device mounting, device discovery
pub mod devices;
/// Search functionality
pub mod search;
