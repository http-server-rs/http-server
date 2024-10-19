mod directory;
mod file;

use std::path::{Component, Path, PathBuf};

use anyhow::Result;
use tokio::fs::OpenOptions;

pub use self::directory::Directory;
pub use self::file::File;

/// Any OS filesystem entry recognized by `ScopedFileSystem` is treated as a
/// `Entry` both `File` and `Directory` are possible values with full support by
/// `ScopedFileSystem`
#[derive(Debug)]
pub enum Entry {
    File(Box<File>),
    Directory(Directory),
}

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

    /// Resolves the provided path against the root directory of this
    /// `ScopedFileSystem` instance.
    ///
    /// A relative path is built using `build_relative_path` and then is opened
    /// to retrieve a `Entry`.
    pub async fn resolve(&self, path: PathBuf) -> std::io::Result<Entry> {
        let entry_path = self.build_relative_path(path);

        Self::open(entry_path).await
    }

    /// Builds a path relative to `ScopedFileSystem`'s `root` path with the
    /// provided path.
    fn build_relative_path(&self, path: PathBuf) -> PathBuf {
        let mut root = self.path.clone();

        root.extend(&Self::normalize_path(&path));

        root
    }

    /// Normalizes paths
    ///
    /// ```ignore
    /// docs/collegue/cs50/lectures/../code/voting_excecise
    /// ```
    ///
    /// Will be normalized to be:
    ///
    /// ```ignore
    /// docs/collegue/cs50/code/voting_excecise
    /// ```
    fn normalize_path(path: &Path) -> PathBuf {
        path.components()
            .fold(PathBuf::new(), |mut result, p| match p {
                Component::ParentDir => {
                    result.pop();
                    result
                }
                Component::Normal(os_string) => {
                    result.push(os_string);
                    result
                }
                _ => result,
            })
    }

    #[cfg(not(target_os = "windows"))]
    async fn open(path: PathBuf) -> std::io::Result<Entry> {
        let mut open_options = OpenOptions::new();
        let entry_path: PathBuf = path.clone();
        let file = open_options.read(true).open(path).await?;
        let metadata = file.metadata().await?;

        if metadata.is_dir() {
            return Ok(Entry::Directory(Directory { path: entry_path }));
        }

        Ok(Entry::File(Box::new(File::new(entry_path, file, metadata))))
    }

    #[cfg(target_os = "windows")]
    async fn open(path: PathBuf) -> std::io::Result<Entry> {
        let mut open_options = OpenOptions::new();
        let entry_path: PathBuf = path.clone();
        let file = open_options
            .read(true)
            .custom_flags(FILE_FLAG_BACKUP_SEMANTICS)
            .open(path)
            .await?;
        let metadata = file.metadata().await?;

        if metadata.is_dir() {
            return Ok(Entry::Directory(Directory { path: entry_path }));
        }

        Ok(Entry::File(Box::new(File::new(entry_path, file, metadata))))
    }
}
