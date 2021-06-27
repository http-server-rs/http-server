use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use mime_guess::{from_path, Mime};
use std::fs::Metadata;
use std::path::PathBuf;

/// Wrapper around `tokio::fs::File` built from a OS ScopedFileSystem file
/// providing `std::fs::Metadata` and the path to such file
#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub file: tokio::fs::File,
    pub metadata: Metadata,
}

impl File {
    pub fn new(path: PathBuf, file: tokio::fs::File, metadata: Metadata) -> Self {
        File {
            path,
            file,
            metadata,
        }
    }

    pub fn mime(&self) -> Mime {
        from_path(self.path.clone()).first_or_octet_stream()
    }

    pub fn size(&self) -> u64 {
        self.metadata.len()
    }

    pub fn last_modified(&self) -> Result<DateTime<Local>> {
        let modified = self
            .metadata
            .modified()
            .context("Failed to read last modified time for file")?;
        let modified: DateTime<Local> = modified.into();

        Ok(modified)
    }
}
