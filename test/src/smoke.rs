use anyhow::{Context, Result};
use reqwest::get;
use xprocess::Process;

use crate::{release_binary_path, wait_on_http_server};

#[tokio::test]
async fn runs_without_panicking() -> Result<()> {
    let http_server = Process::spawn_with_args(release_binary_path()?, ["start"])?;
    wait_on_http_server(7878).await?;
    http_server.kill()?;

    Ok(())
}

#[tokio::test]
async fn returns_json_from_api_index() -> Result<()> {
    let http_server =
        Process::spawn_with_args(release_binary_path()?, ["start", "--port", "7879"])?;
    wait_on_http_server(7879).await?;

    let res = get("http://127.0.0.1:7879/api/v1")
        .await?
        .json::<serde_json::Value>()
        .await?;

    let entries = res
        .get("entries")
        .context("Failed to retrieve entries from JSON.")?;

    let entries = entries
        .as_array()
        .context("Failed to convert entries to array.")?;

    assert!(!entries.is_empty(), "Entries array is empty.");

    http_server.kill()?;
    Ok(())
}
