//! Server-side rendered File Explorer UI built with Topcoat.
//!
//! Everything is embedded in the binary: markup comes from `view!`, the
//! stylesheet and icons are static strings. There is no client-side code.

mod icons;

use std::path::{Path, PathBuf};

use chrono::{DateTime, Local};
use tokio::fs::File;
use tokio::io::AsyncWriteExt;
use topcoat::Result;
use topcoat::context::{Cx, context};
use topcoat::router::content::multipart::Multipart;
use topcoat::router::error::{SeeOther, bad_request, not_found, see_other};
use topcoat::router::{BodyLimit, Router, Slot, layout, page, path_param_segments, route};
use topcoat::view::{View, ViewHandle, view};

use super::core::{Entry, FileExplorer};
use super::listing::index_directory;
use super::proto::{DirectoryEntry, EntryType};
use super::utils::encode_uri;

const STYLES: &str = include_str!("styles.css");

/// Builds the router serving the File Explorer pages for `root`.
pub fn router(root: PathBuf) -> Router {
    Router::builder()
        .with(FileExplorer::new(root))
        .with(BodyLimit::disable())
        .layout(shell)
        .page(index)
        .page(directory)
        .route(upload_root)
        .route(upload)
        .build()
}

fn explorer(cx: &Cx) -> &FileExplorer {
    context(cx)
}

fn svg(markup: &'static str) -> ViewHandle {
    ViewHandle::unescaped_unchecked(markup)
}

fn format_date(date: Option<DateTime<Local>>) -> String {
    date.map(|d| d.format("%Y-%m-%d %H:%M:%S").to_string())
        .unwrap_or_else(|| "Unknown".to_string())
}

fn icon_for(entry_type: &EntryType) -> &'static str {
    match entry_type {
        EntryType::Directory => icons::FOLDER,
        EntryType::Git => icons::GIT,
        EntryType::Justfile => icons::JUSTFILE,
        EntryType::Markdown => icons::MARKDOWN,
        EntryType::Rust => icons::RUST,
        EntryType::Toml => icons::TOML,
        EntryType::File => icons::FILE,
    }
}

#[layout("/")]
async fn shell(cx: &Cx, slot: Slot<'_>) -> Result<impl View> {
    Ok(view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8">
                <meta name="viewport" content="width=device-width, initial-scale=1">
                <title>"File Explorer"</title>
                <style>(ViewHandle::unescaped_unchecked(STYLES))</style>
            </head>
            <body>
                <main>(slot)</main>
            </body>
        </html>
    })
}

#[page("/")]
async fn index(cx: &Cx) -> Result<impl View> {
    explorer_page(cx, Vec::new()).await
}

#[page("/{*path}")]
async fn directory(cx: &Cx) -> Result<impl View> {
    let segments = path_param_segments(cx, "path")
        .map(str::to_owned)
        .collect::<Vec<_>>();

    explorer_page(cx, segments).await
}

async fn explorer_page(cx: &Cx, segments: Vec<String>) -> Result<impl View> {
    let explorer = explorer(cx);
    let dir = match explorer.peek(segments.iter().collect()).await {
        Ok(Entry::Directory(dir)) => dir.path(),
        _ => return Err(not_found().into()),
    };
    let listing = index_directory(explorer.root(), &dir).map_err(|_| not_found())?;

    Ok(view! {
        <div class="bar">
            <form class="upload" method="post" enctype="multipart/form-data">
                <input type="file" name="file" required=(true)>
                <button class="btn" type="submit">"Upload a File"</button>
            </form>
        </div>
        <nav class="crumbs" aria-label="Breadcrumb">
            <ol>
                for crumb in listing.breadcrumbs {
                    <li>
                        if crumb.depth == 0 {
                            <a href=(crumb.entry_link) title="Root">(svg(icons::HOUSE))</a>
                        } else {
                            <a href=(crumb.entry_link)>(crumb.entry_name)</a>
                        }
                    </li>
                }
            </ol>
        </nav>
        <div class="table-wrap">
            <table>
                <thead>
                    <tr>
                        <th></th>
                        <th>"Name"</th>
                        <th>"Size"</th>
                        <th>"Created"</th>
                        <th>"Modified"</th>
                    </tr>
                </thead>
                <tbody>
                    for entry in listing.entries {
                        entry_row(entry: entry)
                    }
                </tbody>
            </table>
        </div>
    })
}

#[topcoat::view::component]
async fn entry_row(cx: &Cx, entry: DirectoryEntry) -> Result<impl View> {
    let download = format!("{}?download", entry.entry_path);

    Ok(view! {
        <tr>
            <td class="icon">(svg(icon_for(&entry.entry_type)))</td>
            <td class="name">
                <a href=(entry.entry_path)>(entry.display_name)</a>
                if !entry.is_dir {
                    <a class="dl" href=(download) title="Download">(svg(icons::DOWNLOAD))</a>
                }
            </td>
            <td class="size">(entry.size_bytes)</td>
            <td>(format_date(entry.date_created))</td>
            <td>(format_date(entry.date_modified))</td>
        </tr>
    })
}

#[route(POST "/")]
async fn upload_root(cx: &Cx, multipart: Multipart) -> Result<SeeOther> {
    store_upload(cx, Vec::new(), multipart).await
}

#[route(POST "/{*path}")]
async fn upload(cx: &Cx, multipart: Multipart) -> Result<SeeOther> {
    let segments = path_param_segments(cx, "path")
        .map(str::to_owned)
        .collect::<Vec<_>>();

    store_upload(cx, segments, multipart).await
}

/// Writes every file field of `multipart` into the directory at `segments`
/// and redirects back to its listing.
async fn store_upload(
    cx: &Cx,
    segments: Vec<String>,
    mut multipart: Multipart,
) -> Result<SeeOther> {
    let explorer = explorer(cx);
    let dir = match explorer.peek(segments.iter().collect()).await {
        Ok(Entry::Directory(dir)) => dir.path(),
        _ => return Err(not_found().into()),
    };

    while let Some(mut field) = multipart.next_field().await? {
        // Only the final path component is trusted, never directories.
        let Some(name) = field
            .file_name()
            .and_then(|name| Path::new(name).file_name())
            .map(|name| name.to_owned())
        else {
            continue;
        };
        let mut file = File::create(dir.join(name))
            .await
            .map_err(|err| bad_request(err.to_string()))?;

        while let Some(chunk) = field.chunk().await? {
            file.write_all(&chunk)
                .await
                .map_err(|err| bad_request(err.to_string()))?;
        }
    }

    let location = if segments.is_empty() {
        String::from("/")
    } else {
        encode_uri(&segments.iter().collect::<PathBuf>())
    };

    Ok(see_other(location))
}
