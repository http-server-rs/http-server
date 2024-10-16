use std::path::PathBuf;

/// Representation of a OS ScopedFileSystem directory providing the path
/// (`PathBuf`)
#[derive(Debug)]
pub struct Directory {
    pub path: PathBuf,
}

impl Directory {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}
