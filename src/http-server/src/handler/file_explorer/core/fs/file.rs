use std::fs::Metadata;
use std::path::PathBuf;

use anyhow::Result;
use mime_guess::{from_path, Mime};
use tokio::io::AsyncReadExt;

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

    pub async fn bytes(&mut self) -> Result<Vec<u8>> {
        let mut buf = Vec::with_capacity(self.size() as usize);
        self.file.read_to_end(&mut buf).await?;
        Ok(buf)
    }
}
