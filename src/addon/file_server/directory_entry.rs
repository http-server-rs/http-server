use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::cmp::{Ord, Ordering};

/// A Directory entry used to display a File Explorer's entry.
/// This struct is directly related to the Handlebars template used
/// to power the File Explorer's UI
#[derive(Debug, Eq, Serialize)]
pub struct DirectoryEntry {
    pub(crate) display_name: String,
    pub(crate) is_dir: bool,
    pub(crate) size_bytes: u64,
    pub(crate) entry_path: String,
    pub(crate) date_created: Option<DateTime<Local>>,
    pub(crate) date_modified: Option<DateTime<Local>>,
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
        Some(self.cmp(other))
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
    pub(crate) sort: Sort,
}

#[derive(Serialize, Debug, PartialEq, Deserialize)]
pub enum Sort {
    Directory,
    Name,
    Size,
    DateCreated,
    DateModified,
}
