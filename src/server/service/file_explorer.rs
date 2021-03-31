use anyhow::{Context, Result};
use chrono::prelude::*;
use chrono::{DateTime, Local};
use handlebars::Handlebars;
use http::response::Builder as HttpResponseBuilder;
use http::StatusCode;
use hyper::{Body, Request, Response};
use hyper_staticfile::{resolve, ResolveResult, ResponseBuilder as FileResponseBuilder};
use serde::Serialize;
use std::fs::read_dir;
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::time::SystemTime;

const EXPLORER_TEMPLATE: &str = "explorer";
const BYTE_SIZE_UNIT: [&str; 9] = ["Bytes", "KB", "MB", "GB", "TB", "PB", "EB", "ZB", "YB"];

#[derive(Debug, Serialize)]
struct DirectoryEntry {
    display_name: String,
    is_dir: bool,
    size: String,
    created_at: String,
    updated_at: String,
}

#[derive(Debug, Serialize)]
struct DirectoryIndex {
    path: String,
    entries: Vec<DirectoryEntry>,
}

#[derive(Clone)]
pub struct FileExplorer<'a> {
    root_dir: PathBuf,
    cache_headers: Option<u32>,
    handlebars: Arc<Handlebars<'a>>,
}

impl<'a> FileExplorer<'a> {
    pub fn new(root_dir: PathBuf) -> Self {
        let handlebars = FileExplorer::make_handlebars_engine();

        FileExplorer {
            root_dir,
            cache_headers: None,
            handlebars,
        }
    }

    fn make_handlebars_engine() -> Arc<Handlebars<'a>> {
        let mut handlebars = Handlebars::new();

        let template = std::include_bytes!("../template/explorer.hbs");
        let template = std::str::from_utf8(template).unwrap();

        handlebars
            .register_template_string(EXPLORER_TEMPLATE, template)
            .unwrap();

        Arc::new(handlebars)
    }

    pub async fn resolve(&self, req: Request<Body>) -> Result<Response<Body>> {
        match resolve(&self.root_dir, &req).await.unwrap() {
            ResolveResult::MethodNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::UriNotMatched => Ok(HttpResponseBuilder::new()
                .status(StatusCode::BAD_REQUEST)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::NotFound => {
                if req.uri() == "/" {
                    let directory_path = self.make_absolute_path_from_request(&req)?;

                    return self.render_directory_index(directory_path).await;
                }

                Ok(HttpResponseBuilder::new()
                    .status(StatusCode::NOT_FOUND)
                    .body(Body::empty())
                    .expect("Failed to build response"))
            }
            ResolveResult::PermissionDenied => Ok(HttpResponseBuilder::new()
                .status(StatusCode::FORBIDDEN)
                .body(Body::empty())
                .expect("Failed to build response")),
            ResolveResult::IsDirectory => {
                let directory_path = self.make_absolute_path_from_request(&req)?;

                return self.render_directory_index(directory_path).await;
            }
            ResolveResult::Found(file, metadata, mime) => Ok(FileResponseBuilder::new()
                .request(&req)
                .cache_headers(self.cache_headers)
                .build(ResolveResult::Found(file, metadata, mime))
                .expect("Failed to build response")),
        }
    }

    async fn render_directory_index(&self, path: PathBuf) -> Result<Response<Body>> {
        let directory_index = FileExplorer::index_directory(path)?;
        let html = self
            .handlebars
            .render(EXPLORER_TEMPLATE, &directory_index)
            .unwrap();

        let body = Body::from(html);

        Ok(HttpResponseBuilder::new()
            .status(StatusCode::OK)
            .body(body)
            .expect("Failed to build response"))
    }

    fn index_directory(path: PathBuf) -> Result<DirectoryIndex> {
        let full_path = path
            .clone()
            .to_str()
            .context("Unable to gather directory path into a String")?
            .to_string();
        let entries = read_dir(path).context("Unable to read directory")?;
        let mut directory_entries: Vec<DirectoryEntry> = Vec::new();

        for entry in entries.into_iter() {
            let entry = entry.context("Unable to read entry")?;
            let metadata = entry.metadata()?;
            let created_at = if let Ok(time) = metadata.created() {
                FileExplorer::format_system_date(time)
            } else {
                String::default()
            };
            let updated_at = if let Ok(time) = metadata.modified() {
                FileExplorer::format_system_date(time)
            } else {
                String::default()
            };

            directory_entries.push(DirectoryEntry {
                display_name: entry
                    .file_name()
                    .to_str()
                    .context("Unable to gather file name into a String")?
                    .to_string(),
                is_dir: metadata.is_dir(),
                size: FileExplorer::format_bytes(metadata.len() as f64),
                created_at,
                updated_at,
            });
        }

        Ok(DirectoryIndex {
            path: full_path,
            entries: directory_entries,
        })
    }

    fn make_absolute_path_from_request(&self, req: &Request<Body>) -> Result<PathBuf> {
        let mut root_dir = self.root_dir.clone();
        let req_path = req.uri().to_string();
        let req_path = if req_path.starts_with("/") {
            let path = req_path[1..req_path.len()].to_string();

            if path.ends_with("/") {
                return PathBuf::from_str(path[..path.len() - 1].to_string().as_str())
                    .context("Unable to buid path");
            }

            PathBuf::from_str(path.as_str())?
        } else {
            PathBuf::from_str(req_path.as_str())?
        };

        root_dir.push(req_path);

        Ok(root_dir)
    }

    fn format_bytes(bytes: f64) -> String {
        if bytes == 0. {
            return String::from("0 Bytes");
        }

        let i = (bytes.log10() / 1024_f64.log10()).floor();
        let value = bytes / 1024_f64.powf(i);

        format!("{:.2} {}", value, BYTE_SIZE_UNIT[i as usize])
    }

    fn format_system_date(system_time: SystemTime) -> String {
        let datetime: DateTime<Local> = DateTime::from(system_time);

        format!(
            "{}/{:0>2}/{:0>2} {:0>2}:{:0>2}:{:0>2}",
            datetime.year(),
            datetime.month(),
            datetime.day(),
            datetime.hour(),
            datetime.minute(),
            datetime.second()
        )
    }
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn format_bytes() {
        let byte_sizes = vec![1024., 1048576., 1073741824., 1099511627776.];

        let expect = vec![
            String::from("1.00 KB"),
            String::from("1.00 MB"),
            String::from("1.00 GB"),
            String::from("1.00 TB"),
        ];

        for (idx, size) in byte_sizes.into_iter().enumerate() {
            assert_eq!(FileExplorer::format_bytes(size), expect[idx]);
        }
    }
}
