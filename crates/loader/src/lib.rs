use anyhow::Result;
use std::{fs, path::Path};

#[derive(Debug)]
pub struct Loader {}

impl Loader {
    pub fn load_from_file<T: AsRef<Path>>(filepath: T) -> Result<String> {
        log::debug!("Loading from file: {:?}", filepath.as_ref());
        let content = fs::read_to_string(filepath)?;
        Ok(content)
    }
    pub async fn load_from_url<T: AsRef<str>>(url: T) -> Result<String> {
        log::debug!("Loading from url: {:?}", url.as_ref());
        let resp = reqwest::get(url.as_ref()).await?;
        let content = resp.text().await?;
        Ok(content)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn load_from_file() {
        let content = Loader::load_from_file("Cargo.toml").unwrap();
        assert!(content.contains("name = \"loader\""));
    }
    #[tokio::test]
    async fn load_from_url() {
        let content = Loader::load_from_url("https://www.baidu.com")
            .await
            .unwrap();
        assert!(content.contains("百度一下"));
    }
}
