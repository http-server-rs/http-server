//! FileSystem wrapper to navigate safely around the children of a root
//! directory.
//!
//! The `ScopedFileSystem` provides read capabilities on a single directory
//! and its children, either files and directories will be accessed by this
//! `ScopedFileSystem` instance.
//!
//! The `Entry` is a wrapper on OS file system entries such as `File` and
//! `Directory`. Both `File` and `Directory` are primitive types for
//! `ScopedFileSystem`
use anyhow::Result;
use std::path::{Component, Path, PathBuf};
use tokio::fs::OpenOptions;

use super::file::File;
use super::QueryParams;

/// The file is being opened or created for a backup or restore operation.
/// The system ensures that the calling process overrides file security
/// checks when the process has SE_BACKUP_NAME and SE_RESTORE_NAME privileges.
/// For more information, see Changing Privileges in a Token.
/// You must set this flag to obtain a handle to a directory.
/// A directory handle can be passed to some functions instead of a file handle.
///
/// Refer: https://docs.microsoft.com/en-us/windows/win32/api/fileapi/nf-fileapi-createfilea
#[cfg(target_os = "windows")]
const FILE_FLAG_BACKUP_SEMANTICS: u32 = 0x02000000;

/// Representation of a OS ScopedFileSystem directory providing the path
/// (`PathBuf`)
#[derive(Debug)]
pub struct Directory {
    path: PathBuf,
}

impl Directory {
    pub fn path(&self) -> PathBuf {
        self.path.clone()
    }
}

/// Any OS filesystem entry recognized by `ScopedFileSystem` is treated as a
/// `Entry` both `File` and `Directory` are possible values with full support by
/// `ScopedFileSystem`
#[derive(Debug)]
pub enum Entry {
    File(Box<File>),
    Directory(Directory),
}

/// `ScopedFileSystem` to resolve and serve static files relative to an specific
/// file system directory
#[derive(Clone)]
pub struct ScopedFileSystem {
    pub root: PathBuf,
}

impl ScopedFileSystem {
    /// Creates a new instance of `ScopedFileSystem` using the provided PathBuf
    /// as the root directory to serve files from.
    ///
    /// Provided paths will resolve relartive to the provided `root` directory.
    pub fn new(root: PathBuf) -> Result<Self> {
        Ok(ScopedFileSystem { root })
    }

    /// Resolves the provided path against the root directory of this
    /// `ScopedFileSystem` instance.
    ///
    /// A relative path is built using `build_relative_path` and then is opened
    /// to retrieve a `Entry`.
    pub async fn resolve(&self, path_and_query: (PathBuf, Option<QueryParams>)) -> std::io::Result<Entry> {
        let entry_path = self.build_relative_path(path_and_query.0);

        ScopedFileSystem::open(entry_path).await
    }

    /// Builds a path relative to `ScopedFileSystem`'s `root` path with the
    /// provided path.
    fn build_relative_path(&self, path: PathBuf) -> PathBuf {
        let mut root = self.root.clone();

        root.extend(&ScopedFileSystem::normalize_path(&path));

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

#[cfg(test)]
mod tests {
    #[allow(unused_imports)]
    use super::*;

    #[test]
    fn builds_a_relative_path_to_root_from_provided_path() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let mut root_path = sfs.root.clone();
        let file_path = PathBuf::from(".github/ISSUE_TEMPLATE/feature-request.md");

        root_path.push(file_path);

        let resolved_path =
            sfs.build_relative_path(PathBuf::from(".github/ISSUE_TEMPLATE/feature-request.md"));

        assert_eq!(root_path, resolved_path);
    }

    #[test]
    fn builds_a_relative_path_to_root_from_forward_slash() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_path = sfs.build_relative_path(PathBuf::from("/"));

        assert_eq!(sfs.root, resolved_path);
    }

    #[test]
    fn builds_a_relative_path_to_root_from_backwards() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_path = sfs.build_relative_path(PathBuf::from("../../"));

        assert_eq!(sfs.root, resolved_path);
    }

    #[test]
    fn builds_an_invalid_path_if_an_arbitrary_path_is_provided() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_path =
            sfs.build_relative_path(PathBuf::from("unexistent_dir/confidential/recipe.txt"));

        assert_ne!(sfs.root, resolved_path);
    }

    #[test]
    fn normalizes_an_arbitrary_path() {
        let arbitrary_path = PathBuf::from("docs/collegue/cs50/lectures/../code/voting_excecise");
        let normalized = ScopedFileSystem::normalize_path(&arbitrary_path.clone());

        assert_eq!(
            normalized.to_str().unwrap(),
            "docs/collegue/cs50/code/voting_excecise"
        );
    }

    #[tokio::test]
    async fn resolves_a_file() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_entry = sfs.resolve((PathBuf::from("assets/logo.svg"), None)).await.unwrap();

        if let Entry::File(file) = resolved_entry {
            assert_eq!(file.metadata.is_file(), true);
        } else {
            panic!("Found a directory instead of a file in the provied path");
        }
    }

    #[tokio::test]
    async fn detect_directory_paths() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_entry = sfs.resolve((PathBuf::from("assets/"), None)).await.unwrap();

        assert!(matches!(resolved_entry, Entry::Directory(_)));
    }

    #[tokio::test]
    async fn detect_directory_paths_without_postfixed_slash() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_entry = sfs.resolve((PathBuf::from("assets"), None)).await.unwrap();

        assert!(matches!(resolved_entry, Entry::Directory(_)));
    }

    #[tokio::test]
    async fn returns_error_if_file_doesnt_exists() {
        let sfs = ScopedFileSystem::new(PathBuf::from("")).unwrap();
        let resolved_entry = sfs
            .resolve((PathBuf::from("assets/unexistent_file.doc"), None))
            .await;

        assert_eq!(resolved_entry.is_err(), true);
    }
}
