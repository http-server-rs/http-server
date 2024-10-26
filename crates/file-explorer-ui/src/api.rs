use anyhow::Result;
use reqwest::Url;

use file_explorer_proto::DirectoryIndex;

pub struct Api {
    base_url: Url,
}

impl Api {
    pub fn new() -> Self {
        let base_url = Url::parse("http://localhost:3000").unwrap();

        Self { base_url }
    }

    pub async fn peek(&self, path: &str) -> Result<DirectoryIndex> {
        let url = self.base_url.join(&format!("api/v1/{path}"))?;
        let index = reqwest::get(url).await?.json::<DirectoryIndex>().await?;

        Ok(index)
    }
}
