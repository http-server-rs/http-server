pub mod directory;
pub mod file;

use std::path::PathBuf;

use anyhow::Result;

#[derive(Debug, Clone)]
pub struct FileSystem {
    pub path: PathBuf,
}

impl FileSystem {
    /// Creates a new instance of `ScopedFileSystem` using the provided PathBuf
    /// as the root directory to serve files from.
    ///
    /// Provided paths will resolve relartive to the provided `root` directory.
    pub fn new(path: PathBuf) -> Result<Self> {
        Ok(Self { path })
    }
}
