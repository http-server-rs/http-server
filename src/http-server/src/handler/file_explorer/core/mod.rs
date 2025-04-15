mod fs;

use std::path::{Component, Path, PathBuf};

use anyhow::Result;
use tokio::fs::OpenOptions;

use self::fs::directory::Directory;
use self::fs::file::File;

/// Any OS filesystem entry recognized by [`FileExplorer`] is treated as a
/// `Entry` both `File` and `Directory` are possible values.
#[derive(Debug)]
pub enum Entry {
    File(Box<File>),
    Directory(Directory),
}

pub struct FileExplorer {
    root: PathBuf,
}

impl FileExplorer {
    pub fn new(root: PathBuf) -> Self {
        Self { root }
    }

    /// Peeks on the provided `path` as a "subpath" for this [`FileExplorer`] instance.
    pub async fn peek(&self, path: PathBuf) -> Result<Entry> {
        let relative_path = self.build_relative_path(path);
        self.open(relative_path).await
    }

    /// Joins the provided `path` with the `root` path of this [`FileExplorer`] instance.
    fn build_relative_path(&self, path: PathBuf) -> PathBuf {
        let mut root = self.root.clone();
        root.extend(&self.normalize_path(&path));
        root
    }

    /// Normalizes a `Path` to be directory-traversal safe.
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
    ///
    /// # Reference
    ///
    /// - https://owasp.org/www-community/attacks/Path_Traversal
    fn normalize_path(&self, path: &Path) -> PathBuf {
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
    async fn open(&self, path: PathBuf) -> Result<Entry> {
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
    async fn open(&self, path: PathBuf) -> Result<Entry> {
        /// The file is being opened or created for a backup or restore operation.
        /// The system ensures that the calling process overrides file security
        /// checks when the process has SE_BACKUP_NAME and SE_RESTORE_NAME privileges.
        ///
        /// For more information, see Changing Privileges in a Token.
        /// You must set this flag to obtain a handle to a directory.
        /// A directory handle can be passed to some functions instead of a file handle.
        ///
        /// Refer: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
        const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

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
