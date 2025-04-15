use std::cmp::Ordering;

use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
pub enum EntryType {
    Directory,
    File,
    Git,
    Justfile,
    Markdown,
    Rust,
    Toml,
}

/// A Directory entry used to display a File Explorer's entry.
/// This struct is directly related to the Handlebars template used
/// to power the File Explorer's UI
#[derive(Clone, Debug, Deserialize, Eq, Serialize)]
pub struct DirectoryEntry {
    pub display_name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub entry_path: String,
    pub entry_type: EntryType,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
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
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub depth: u8,
    pub entry_name: String,
    pub entry_link: String,
}

/// The value passed to the Handlebars template engine.
/// All references contained in File Explorer's UI are provided
/// via the `DirectoryIndex` struct
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct DirectoryIndex {
    pub entries: Vec<DirectoryEntry>,
    pub breadcrumbs: Vec<BreadcrumbItem>,
    pub sort: Sort,
}

#[derive(Clone, Serialize, Debug, PartialEq, Deserialize)]
pub enum Sort {
    Directory,
    Name,
    Size,
    DateCreated,
    DateModified,
}
