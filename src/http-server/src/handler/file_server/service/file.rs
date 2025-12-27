use std::fs::Metadata;
use std::mem::MaybeUninit;
use std::path::PathBuf;
use std::pin::Pin;
use std::task::{self, Poll};

use anyhow::{Context, Result};
use chrono::{DateTime, Local};
use futures::Stream;
use hyper::body::Bytes;
use mime_guess::{Mime, from_path};
use tokio::io::{AsyncRead, ReadBuf};

pub const FILE_BUFFER_SIZE: usize = 8 * 1024;

pub type FileBuffer = Box<[MaybeUninit<u8>; FILE_BUFFER_SIZE]>;

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
