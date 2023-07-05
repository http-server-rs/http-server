//! File System utilities for the file explorer

use std::fs::Metadata;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{self, Poll};

use axum::body::Bytes;
use chrono::{DateTime, Local};
use futures::Stream;
use mime_guess::{from_path, Mime};
#[allow(unused_imports)]
use tokio::fs::OpenOptions;
use tokio::io::{AsyncRead, ReadBuf};

use crate::Result;

pub const FILE_BUFFER_SIZE: usize = 8 * 1024;

pub type FileBuffer = Box<[MaybeUninit<u8>; FILE_BUFFER_SIZE]>;

/// Wrapper around `tokio::fs::File` providing `std::fs::Metadata` and the
/// path to the file
#[derive(Debug)]
pub struct File {
    pub path: PathBuf,
    pub file: tokio::fs::File,
    pub metadata: Metadata,
}

#[allow(dead_code)]
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
        let modified = self.metadata.modified()?;
        let modified: DateTime<Local> = modified.into();

        Ok(modified)
    }

    #[allow(dead_code)]
    pub fn bytes(self) -> Vec<u8> {
        let byte_stream = ByteStream {
            file: self.file,
            buffer: Box::new([MaybeUninit::uninit(); FILE_BUFFER_SIZE]),
        };

        byte_stream
            .buffer
            .iter()
            .map(|muint| unsafe { muint.assume_init() })
            .collect::<Vec<u8>>()
    }
}

pub struct ByteStream {
    file: tokio::fs::File,
    buffer: FileBuffer,
}

impl From<File> for ByteStream {
    fn from(file: File) -> Self {
        ByteStream {
            file: file.file,
            buffer: Box::new([MaybeUninit::uninit(); FILE_BUFFER_SIZE]),
        }
    }
}

impl Stream for ByteStream {
    type Item = Result<Bytes>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Option<Self::Item>> {
        let ByteStream {
            ref mut file,
            ref mut buffer,
        } = *self;
        let mut read_buffer = ReadBuf::uninit(&mut buffer[..]);

        match Pin::new(file).poll_read(cx, &mut read_buffer) {
            Poll::Ready(Ok(())) => {
                let filled = read_buffer.filled();

                if filled.is_empty() {
                    Poll::Ready(None)
                } else {
                    Poll::Ready(Some(Ok(Bytes::copy_from_slice(filled))))
                }
            }
            Poll::Ready(Err(error)) => Poll::Ready(Some(Err(error.into()))),
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Representation of a directory
#[derive(Debug)]
pub struct Directory {
    #[allow(dead_code)]
    path: PathBuf,
}

/// An entry in a directory
#[allow(dead_code)]
#[derive(Debug)]
pub enum Entry {
    File(Box<File>),
    Directory(Directory),
}

#[cfg(not(target_os = "windows"))]
pub async fn open(path: PathBuf) -> Result<Entry> {
    let mut open_options = OpenOptions::new();
    let file = open_options.read(true).open(&path).await?;
    let metadata = file.metadata().await?;

    if metadata.is_dir() {
        return Ok(Entry::Directory(Directory { path }));
    }

    Ok(Entry::File(Box::new(File::new(path, file, metadata))))
}
#[allow(unused_variables)]
#[cfg(target_os = "windows")]
pub async fn open(path: PathBuf) -> Result<Entry> {
    todo!("Windows support is not yet implemented")
}
