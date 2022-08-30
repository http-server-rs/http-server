use serde::Serialize;
use std::cmp::{Ord, Ordering};

/// A Directory entry used to display a File Explorer's entry.
/// This struct is directly related to the Handlebars template used
/// to power the File Explorer's UI
#[derive(Debug, Eq, Serialize)]
pub struct DirectoryEntry {
    pub(crate) display_name: String,
    pub(crate) is_dir: bool,
    pub(crate) size: String,
    pub(crate) len: u64,
    pub(crate) entry_path: String,
    pub(crate) created_at: String,
    pub(crate) updated_at: String,
}

impl Ord for DirectoryEntry {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.is_dir && other.is_dir {
            return self.display_name.cmp(&other.display_name);
        }

        if self.is_dir && !other.is_dir {
            return Ordering::Less;
        }

        Ordering::Greater
    }
}

impl PartialOrd for DirectoryEntry {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.is_dir && other.is_dir {
            return Some(self.display_name.cmp(&other.display_name));
        }

        if self.is_dir && !other.is_dir {
            return Some(Ordering::Less);
        }

        Some(Ordering::Greater)
    }
}

impl PartialEq for DirectoryEntry {
    fn eq(&self, other: &Self) -> bool {
        if self.is_dir && other.is_dir {
            return self.display_name == other.display_name;
        }

        self.display_name == other.display_name
    }
}

/// A Breadcrumb Item used to navigate to previous path components
#[derive(Clone, Debug, Serialize, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub(crate) entry_name: String,
    pub(crate) entry_link: String,
}

/// The value passed to the Handlebars template engine.
/// All references contained in File Explorer's UI are provided
/// via the `DirectoryIndex` struct
#[derive(Debug, Serialize)]
pub struct DirectoryIndex {
    /// Directory listing entry
    pub(crate) entries: Vec<DirectoryEntry>,
    pub(crate) breadcrumbs: Vec<BreadcrumbItem>,
    pub(crate) sort_by_name: bool,
    pub(crate) sort_by_size: bool,
}
