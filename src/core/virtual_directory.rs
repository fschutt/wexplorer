//! A `VirtualDirectory` is in most cases a wrapper around a directory
//! However, it can contain multiple `PathBufs` that are not necessarily in the same
//! physical directory.

use std::path::PathBuf;

pub struct VirtualDirectory {
    pub paths: Vec<PathBuf>,
}
