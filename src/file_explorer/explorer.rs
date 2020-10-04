use crate::file_explorer::{Entry, Error};
use std::io::ErrorKind;
use std::env::current_dir;
use std::fs::metadata;
use std::path::PathBuf;
use std::str::FromStr;

/// A file explorer to navigate around a particular directory (`root_dir`),
/// exposing methods to operate in this directory.
///
/// The `root_dir` is the base directory for every operation
/// for this `FileExplorer`.
///
/// A file explorer manages operations in an specific directory.
/// Its able to read, create and delete files (`Entry` represents each
/// file or directory).
///
/// Is not possible to _move out_ of the `root_dir`, this means that
/// every opertation executed through the `FileExplorer` must be
/// done in a path relative to the explorer.
///
/// The root (`/`) path is equivalent to `root_dir` in a `FileExplorer` instance.
/// For example, if the `root_dir` equals to `~/docs/music` then `/` is equivalent to
/// `~/docs/music` and `/favorites` equivalent to `~/docs/music/favorite`.
pub struct FileExplorer {
    pub root_dir: PathBuf,
    pub root_dir_string: String,
}

impl FileExplorer {
    /// Creates a new `FileExplorer` instance.
    pub fn new(root_dir: PathBuf) -> Self {
        let mut final_path = current_dir().unwrap();

        final_path.push(root_dir);

        Self {
            root_dir: final_path.clone(),
            root_dir_string: final_path.to_str().unwrap().to_string()
        }
    }

    /// Retrieves an `Entry` from the provided `path`
    pub fn read(&self, path: &str) -> Result<Entry, Error> {
        let path = self.build_relative_path(path);

        match metadata(path.clone()) {
            Ok(meta) => Ok(Entry::new(path, meta)),
            Err(err) => {
                match err.kind() {
                    ErrorKind::NotFound => Err(Error::NoExists(path.to_str().unwrap().to_string())),
                    _ => Err(Error::IOError(err)),
                }
            }
        }
    }

    pub fn to_relative_path(&self, path: &str) -> Result<String, Error> {
        // if this path part of the `root_dir`
        if path.contains(self.root_dir.to_str().unwrap()) {
            let root_path_length = self.root_dir.to_str().unwrap().len();
            let relative = path[root_path_length..].to_string();

            return Ok(relative);
        }

        Err(Error::NoExists(path.to_string()))
    }

    /// Builds a path relative to `root_dir` and returns it
    fn build_relative_path(&self, path: &str) -> PathBuf {
        if path == "/" {
            // the path is `/` so we return the `root_dir`
            // for this `FileExplorer` instance
            return self.root_dir.clone();
        }

        // retrieve every fragment of the path (directory).
        let sanitized_path = path.split('/')
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|&e| e != "")
            .collect::<Vec<&str>>()
            .join("/");

        let mut next_path = self.root_dir.clone();
        
        next_path.push(PathBuf::from_str(&sanitized_path).unwrap());

        next_path
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_builds_root_path_when_a_slash_is_provided() {
        let file_explorer = FileExplorer::new(PathBuf::from_str("").unwrap());
        let relative = file_explorer.build_relative_path("/");

        assert_eq!(file_explorer.root_dir.to_str().unwrap(), relative.to_str().unwrap());
    }

    #[test]
    fn it_builds_relative_to_root_path() {
        let file_explorer = FileExplorer::new(PathBuf::from_str("").unwrap());
        let mut root_dir_on_music = file_explorer.root_dir.clone();

        root_dir_on_music.push("music/favorites");

        let relative = file_explorer.build_relative_path("/music/favorites");

        assert_eq!(root_dir_on_music.to_str().unwrap(), relative.to_str().unwrap());
    }

    #[test]
    fn it_fixes_and_appends_invalid_paths() {
        let file_explorer = FileExplorer::new(PathBuf::from_str("").unwrap());
        let mut root_dir_on_music = file_explorer.root_dir.clone();

        root_dir_on_music.push("music/favorites");

        let relative = file_explorer.build_relative_path("//music/favorites");

        assert_eq!(root_dir_on_music.to_str().unwrap(), relative.to_str().unwrap());
    }
}
