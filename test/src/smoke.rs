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
async fn renders_html_file_explorer() -> Result<()> {
    let http_server =
        Process::spawn_with_args(release_binary_path()?, ["start", "--port", "7879"])?;
    wait_on_http_server(7879).await?;

    let res = get("http://127.0.0.1:7879/").await?;
    let content_type = res
        .headers()
        .get("content-type")
        .context("Missing content-type header.")?
        .to_str()?
        .to_string();
    let body = res.text().await?;

    assert!(content_type.starts_with("text/html"), "{content_type}");
    assert!(body.contains("<table>"), "Directory listing table missing.");
    assert!(body.contains("Cargo.toml"), "Expected entry missing.");

    http_server.kill()?;
    Ok(())
}
