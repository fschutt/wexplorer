//! Centralized struct holding all necessary information to run the application.

use std::path::PathBuf;

use core::file_info::FileInfo;
 
pub struct App {
    pub current_path: PathBuf,
    pub current_file_infos: Vec<FileInfo>,
}
