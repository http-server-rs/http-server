use std::fs::{DirEntry, Metadata};
use std::path::PathBuf;
use std::time::SystemTime;
use std::cmp::Ordering;

/// `FileExplorer` entry containing it's path (`PathBuf`) and
/// it's `Metadata`
#[derive(Debug, PartialEq, Eq)]
pub struct Entry {
    pub path: PathBuf,
    pub is_file: bool,
    pub size: u64,
    pub created_at: Option<SystemTime>,
    pub updated_at: Option<SystemTime>,
}

impl Entry {
    pub fn new(path: PathBuf, meta: Metadata) -> Self {
        let is_file = meta.is_file();
        let mut size = 0;
        let mut created_at = None;
        let mut updated_at = None;

        if is_file {
            size = meta.len();
            created_at = Some(meta.created().unwrap());
            updated_at = Some(meta.accessed().unwrap());
        }

        Self {
            path,
            is_file,
            size,
            created_at,
            updated_at,
        }
    }
}

impl Ord for Entry {
    fn cmp(&self, _: &Entry) -> Ordering {
        if self.is_file {
            return Ordering::Greater;
        }

        Ordering::Less
    }
}

impl PartialOrd for Entry {
    fn partial_cmp(&self, other: &Entry) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<DirEntry> for Entry {
    fn from(dir_entry: DirEntry) -> Self {
        Self::new(dir_entry.path(), dir_entry.metadata().unwrap())
    }
}
