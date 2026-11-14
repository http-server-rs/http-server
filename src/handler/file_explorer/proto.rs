use chrono::{DateTime, Local};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EntryType {
    Directory,
    File,
    Git,
    Justfile,
    Markdown,
    Rust,
    Toml,
}

/// A directory entry displayed as a row in the File Explorer.
#[derive(Clone, Debug)]
pub struct DirectoryEntry {
    pub display_name: String,
    pub is_dir: bool,
    pub size_bytes: u64,
    pub entry_path: String,
    pub entry_type: EntryType,
    pub date_created: Option<DateTime<Local>>,
    pub date_modified: Option<DateTime<Local>>,
}

/// A breadcrumb item used to navigate to previous path components.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct BreadcrumbItem {
    pub depth: u8,
    pub entry_name: String,
    pub entry_link: String,
}

/// Everything the File Explorer page needs to render a directory.
#[derive(Clone, Debug)]
pub struct DirectoryIndex {
    pub entries: Vec<DirectoryEntry>,
    pub breadcrumbs: Vec<BreadcrumbItem>,
}
