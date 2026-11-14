use std::fs::read_dir;
use std::path::{Component, Path};

use anyhow::{Context, Result};
use percent_encoding::{percent_decode_str, utf8_percent_encode};

use super::proto::{BreadcrumbItem, DirectoryEntry, DirectoryIndex, EntryType};
use super::utils::{PERCENT_ENCODE_SET, encode_uri};

/// Encodes a `Path` component using `PercentEncode` with UTF-8 charset.
fn encode_component(comp: Component) -> String {
    let component = comp.as_os_str().to_string_lossy();

    utf8_percent_encode(&component, PERCENT_ENCODE_SET).to_string()
}

fn breadcrumbs_from_path(root_dir: &Path, path: &Path) -> Result<Vec<BreadcrumbItem>> {
    let root_dir_name = root_dir
        .components()
        .next_back()
        .map(|c| c.as_os_str().to_string_lossy().to_string())
        .unwrap_or_default();
    let stripped = path
        .strip_prefix(root_dir)?
        .components()
        .map(encode_component)
        .collect::<Vec<String>>();

    let mut breadcrumbs = stripped
        .iter()
        .enumerate()
        .map(|(idx, entry_name)| BreadcrumbItem {
            depth: (idx + 1) as u8,
            entry_name: percent_decode_str(entry_name)
                .decode_utf8_lossy()
                .to_string(),
            entry_link: format!("/{}", stripped[0..=idx].join("/")),
        })
        .collect::<Vec<BreadcrumbItem>>();

    breadcrumbs.insert(
        0,
        BreadcrumbItem {
            depth: 0,
            entry_name: root_dir_name,
            entry_link: String::from("/"),
        },
    );

    Ok(breadcrumbs)
}

fn entry_type_for(display_name: &str, is_dir: bool) -> EntryType {
    if is_dir {
        return EntryType::Directory;
    }

    match display_name
        .rsplit('.')
        .next()
        .unwrap_or_default()
        .to_ascii_lowercase()
        .as_str()
    {
        "gitignore" | "gitkeep" => EntryType::Git,
        "justfile" => EntryType::Justfile,
        "md" => EntryType::Markdown,
        "rs" => EntryType::Rust,
        "toml" => EntryType::Toml,
        _ => EntryType::File,
    }
}

/// Creates a `DirectoryIndex` for `path` (an absolute or root-relative
/// directory inside `root_dir`).
pub fn index_directory(root_dir: &Path, path: &Path) -> Result<DirectoryIndex> {
    let breadcrumbs = breadcrumbs_from_path(root_dir, path)?;
    let mut entries = Vec::new();

    for entry in read_dir(path).context("Unable to read directory")? {
        let entry = entry.context("Unable to read entry")?;
        let metadata = entry.metadata()?;
        let display_name = entry.file_name().to_string_lossy().to_string();
        let entry_path = encode_uri(
            entry
                .path()
                .strip_prefix(root_dir)
                .context("Entry is outside of the root directory")?,
        );

        entries.push(DirectoryEntry {
            is_dir: metadata.is_dir(),
            size_bytes: metadata.len(),
            entry_path,
            entry_type: entry_type_for(&display_name, metadata.is_dir()),
            display_name,
            date_created: metadata.created().ok().map(Into::into),
            date_modified: metadata.modified().ok().map(Into::into),
        });
    }

    entries.sort_by(|a, b| {
        b.is_dir.cmp(&a.is_dir).then_with(|| {
            a.display_name
                .to_lowercase()
                .cmp(&b.display_name.to_lowercase())
        })
    });

    Ok(DirectoryIndex {
        entries,
        breadcrumbs,
    })
}
