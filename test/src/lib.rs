#[cfg(test)]
mod smoke;

use std::{env::var, str::FromStr, time::Duration};

use anyhow::Result;
use reqwest::{Method, Url};
use wait_on::{WaitOptions, Waitable, resource::http::HttpWaiter};

pub fn release_binary_path() -> Result<String> {
    let path = var("TARGET")?;
    Ok(format!("../target/{path}/release/http-server"))
}

pub async fn wait_on_http_server(port: u16) -> Result<()> {
    let url = Url::from_str(&format!("http://127.0.0.1:{port}"))?;
    let task = HttpWaiter::new(Method::GET, url);
    task.wait(&WaitOptions {
        timeout: Duration::from_secs(10),
    })
    .await?;
    Ok(())
}
