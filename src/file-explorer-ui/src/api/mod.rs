pub mod proto;

use anyhow::Result;
use gloo::utils::window;
use reqwest::{header::CONTENT_TYPE, Client, Url};
use web_sys::File;

use self::proto::DirectoryIndex;

pub struct FileDownload {
    pub bytes: Vec<u8>,
    pub mime: String,
}

pub struct Api {
    base_url: Url,
}

impl Api {
    pub fn new() -> Self {
        let base_url = Url::parse(&window().location().href().unwrap()).unwrap();

        Self { base_url }
    }

    pub async fn peek(&self, path: &str) -> Result<DirectoryIndex> {
        let path = path.strip_prefix("/").unwrap();
        let url = self.base_url.join(&format!("/api/v1/{path}"))?;
        let index = reqwest::get(url).await?.json::<DirectoryIndex>().await?;

        Ok(index)
    }

    pub async fn upload(&self, file: File) -> Result<()> {
        let file_name = file.name();
        let reader = gloo_file::futures::read_as_bytes(&file.into()).await?;

        let url = self.base_url.join("api/v1")?;
        let _response = Client::new()
            .post(url.as_ref())
            .header("Content-Type", "application/octet-stream")
            .header("X-File-Name", file_name)
            .body(reader)
            .send()
            .await?;
        // let form_data = FormData::new()
        //     .map_err(|err| Error::msg(format!("Failed to create FormData: {:?}", err)))?;
        // form_data
        //     .append_with_blob("file", &file)
        //     .map_err(|err| Error::msg(format!("Failed to append file to FormData: {:?}", err)))?;

        // gloo::net::http::Request::post(url.as_ref())
        //     .body(form_data)?
        //     .send()
        //     .await?;

        Ok(())
    }

    pub async fn download(&self, path: &str) -> Result<FileDownload> {
        let path = path.strip_prefix("/").unwrap();
        let url = self.base_url.join(&format!("/api/v1/{path}"))?;
        let res = reqwest::get(url).await?;
        let headers = res.headers();
        let mime = headers
            .get(CONTENT_TYPE)
            .map(|hv| hv.to_str().unwrap().to_string())
            .unwrap_or("application/octet-stream".to_string());
        let bytes = res.bytes().await?.to_vec();

        Ok(FileDownload { bytes, mime })
    }
}
