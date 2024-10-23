mod fs;
mod templater;
mod utils;

use std::cmp::{Ord, Ordering};
use std::fs::read_dir;
use std::mem::MaybeUninit;
use std::path::{Component, Path, PathBuf};
use std::sync::Arc;

use anyhow::{Context, Result};
use async_trait::async_trait;
use chrono::{DateTime, Local};
use fs::Entry;
use http::request::Parts;
use http_body_util::Full;
use hyper::body::Bytes;
use hyper::header::{CONTENT_TYPE, ETAG, LAST_MODIFIED};
use hyper::{Method, Response, StatusCode, Uri};
use percent_encoding::{percent_decode_str, utf8_percent_encode};
use serde::{Deserialize, Serialize};
use tokio::io::AsyncWriteExt;
use tokio::runtime::Handle;

use http_server_plugin::config::read_from_path;
use http_server_plugin::{export_plugin, Function, InvocationError, PluginRegistrar};

use self::fs::{File, FileSystem};
use self::templater::Templater;
use self::utils::{decode_uri, encode_uri, PERCENT_ENCODE_SET};

const FILE_BUFFER_SIZE: usize = 8 * 1024;

pub type FileBuffer = Box<[MaybeUninit<u8>; FILE_BUFFER_SIZE]>;

export_plugin!(register);

const PLUGIN_NAME: &str = "file-explorer";

#[allow(improper_ctypes_definitions)]
extern "C" fn register(config_path: PathBuf, rt: Arc<Handle>, registrar: &mut dyn PluginRegistrar) {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    let config: FileExplorerConfig = read_from_path(config_path, PLUGIN_NAME).unwrap();

    registrar.register_function(
        PLUGIN_NAME,
        Arc::new(FileExplorer::new(rt, config.path).expect("Failed to create FileExplorer")),
    );
}

#[derive(Debug, Deserialize)]
struct FileExplorerConfig {
    pub path: PathBuf,
}

struct FileExplorer {
    rt: Arc<Handle>,
    fs: FileSystem,
    path: PathBuf,
    templater: Templater,
}

#[async_trait]
impl Function for FileExplorer {
    async fn call(
        &self,
        parts: Parts,
        body: Bytes,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        self.rt
            .block_on(async move { self.handle(parts, body).await })
    }
}

impl FileExplorer {
    fn new(rt: Arc<Handle>, path: PathBuf) -> Result<Self> {
        let fs = FileSystem::new(path.clone())?;
        let templater = Templater::new()?;

        Ok(Self {
            rt,
            fs,
            path,
            templater,
        })
    }

    async fn handle(
        &self,
        parts: Parts,
        body: Bytes,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        if parts.uri.path().starts_with("/api/v1") {
            self.handle_api(parts, body).await
        } else {
            Ok(Response::new(Full::new(Bytes::from("Unsupported method"))))
        }
    }

    async fn handle_api(
        &self,
        parts: Parts,
        body: Bytes,
    ) -> Result<Response<Full<Bytes>>, InvocationError> {
        let path = Self::parse_req_uri(parts.uri).unwrap();

        match parts.method {
            Method::GET => match self.fs.resolve(path).await {
                Ok(entry) => match entry {
                    Entry::Directory(dir) => {
                        Ok(self.render_directory_index(dir.path()).await.unwrap())
                    }
                    Entry::File(file) => Ok(Self::make_http_file_response(file).await.unwrap()),
                },
                Err(err) => {
                    let message = format!("Failed to resolve path: {}", err);
                    Ok(Response::new(Full::new(Bytes::from(message))))
                }
            },
            Method::POST => {
                let filename = path.file_name().unwrap().to_str().unwrap();
                let mut file = tokio::fs::File::create(filename).await.unwrap();
                file.write_all(&body).await.unwrap();
                Ok(Response::new(Full::new(Bytes::from(
                    "POST method is not supported",
                ))))
            }
            _ => Ok(Response::new(Full::new(Bytes::from("Unsupported method")))),
        }
    }

    fn parse_req_uri(uri: Uri) -> Result<PathBuf> {
        let parts: Vec<&str> = uri.path().split('/').collect();
        let path = &parts[3..].join("/");

        Ok(decode_uri(path))
    }

    /// Encodes a `PathBuf` component using `PercentEncode` with UTF-8 charset.
    ///
    /// # Panics
    ///
    /// If the component's `OsStr` representation doesn't belong to valid UTF-8
    /// this function panics.
    fn encode_component(comp: Component) -> String {
        let component = comp
            .as_os_str()
            .to_str()
            .expect("The provided OsStr doesn't belong to the UTF-8 charset.");

        utf8_percent_encode(component, PERCENT_ENCODE_SET).to_string()
    }

    fn breadcrumbs_from_path(root_dir: &Path, path: &Path) -> Result<Vec<BreadcrumbItem>> {
        let root_dir_name = root_dir
            .components()
            .last()
            .unwrap()
            .as_os_str()
            .to_str()
            .expect("The first path component is not UTF-8 charset compliant.");
        let stripped = path
            .strip_prefix(root_dir)?
            .components()
            .map(Self::encode_component)
            .collect::<Vec<String>>();

        let mut breadcrumbs = stripped
            .iter()
            .enumerate()
            .map(|(idx, entry_name)| BreadcrumbItem {
                entry_name: percent_decode_str(entry_name)
                    .decode_utf8()
                    .expect("The path name is not UTF-8 compliant")
                    .to_string(),
                entry_link: format!("/{}", stripped[0..=idx].join("/")),
            })
            .collect::<Vec<BreadcrumbItem>>();

        breadcrumbs.insert(
            0,
            BreadcrumbItem {
                entry_name: String::from(root_dir_name),
                entry_link: String::from("/"),
            },
        );

        Ok(breadcrumbs)
    }

    /// Creates entry's relative path. Used by Handlebars template engine to
    /// provide navigation through `FileExplorer`
    ///
    /// If the root_dir is: `https-server/src`
    /// The entry path is: `https-server/src/server/service/file_explorer.rs`
    ///
    /// Then the resulting path from this function is the absolute path to
    /// the "entry path" in relation to the "root_dir" path.
    ///
    /// This happens because links should behave relative to the `/` path
    /// which in this case is `http-server/src` instead of system's root path.
    fn make_dir_entry_link(root_dir: &Path, entry_path: &Path) -> String {
        let path = entry_path.strip_prefix(root_dir).unwrap();

        encode_uri(path)
    }

    /// Creates a `DirectoryIndex` with the provided `root_dir` and `path`
    /// (HTTP Request URI)
    fn index_directory(root_dir: PathBuf, path: PathBuf) -> Result<DirectoryIndex> {
        let breadcrumbs = Self::breadcrumbs_from_path(&root_dir, &path)?;
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;
            let date_created = if let Ok(time) = metadata.created() {
                Some(time.into())
            } else {
                None
            };
            let date_modified = if let Ok(time) = metadata.modified() {
                Some(time.into())
            } else {
                None
            };

            directory_entries.push(DirectoryEntry {
                display_name: entry
                    .file_name()
                    .to_str()
                    .context("Unable to gather file name into a String")?
                    .to_string(),
                is_dir: metadata.is_dir(),
                size_bytes: metadata.len(),
                entry_path: Self::make_dir_entry_link(&root_dir, &entry.path()),
                date_created,
                date_modified,
            });
        }

        // if let Some(query_params) = query_params {
        //     if let Some(sort_by) = query_params.sort_by {
        //         match sort_by {
        //             SortBy::Name => {
        //                 directory_entries.sort_by_key(|entry| entry.display_name.clone());
        //             }
        //             SortBy::Size => directory_entries.sort_by_key(|entry| entry.size_bytes),
        //             SortBy::DateCreated => {
        //                 directory_entries.sort_by_key(|entry| entry.date_created)
        //             }
        //             SortBy::DateModified => {
        //                 directory_entries.sort_by_key(|entry| entry.date_modified)
        //             }
        //         };

        //         let sort_enum = match sort_by {
        //             SortBy::Name => Sort::Name,
        //             SortBy::Size => Sort::Size,
        //             SortBy::DateCreated => Sort::DateCreated,
        //             SortBy::DateModified => Sort::DateModified,
        //         };

        //         return Ok(DirectoryIndex {
        //             entries: directory_entries,
        //             breadcrumbs,
        //             sort: sort_enum,
        //         });
        //     }
        // }

        directory_entries.sort();

        Ok(DirectoryIndex {
            entries: directory_entries,
            breadcrumbs,
            sort: Sort::Directory,
        })
    }

    /// Indexes the directory by creating a `DirectoryIndex`. Such `DirectoryIndex`
    /// is used to build the Handlebars "Explorer" template using the Handlebars
    /// engine and builds an HTTP Response containing such file
    async fn render_directory_index(&self, path: PathBuf) -> Result<Response<Full<Bytes>>> {
        let directory_index = Self::index_directory(self.path.clone(), path)?;
        let html = self.templater.render(&directory_index).unwrap();

        Response::builder()
            .header(CONTENT_TYPE, "text/html")
            .status(StatusCode::OK)
            .body(Full::new(Bytes::from(html)))
            .context("Failed to build response")
    }

    pub async fn make_http_file_response(file: Box<File>) -> Result<Response<Full<Bytes>>> {
        Response::builder()
            .header(CONTENT_TYPE, file.mime().to_string())
            .header(
                ETAG,
                format!(
                    "W/\"{0:x}-{1:x}.{2:x}\"",
                    file.size(),
                    file.last_modified().unwrap().timestamp(),
                    file.last_modified().unwrap().timestamp_subsec_nanos(),
                ),
            )
            .header(LAST_MODIFIED, file.last_modified().unwrap().to_rfc2822())
            .body(Full::new(Bytes::from(file.bytes())))
            .context("Failed to build HTTP File Response")
    }
}

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
