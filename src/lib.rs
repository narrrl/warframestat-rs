pub mod model;

use reqwest::Client as HttpClient;
use url::Url;

const API_URL: &'static str = "https://api-warframestat.us";

///
/// WarframeClient to request data from the [Warframestat API](https://doc.warframestat.us)
///
#[derive(Debug)]
pub struct WarframeClient {
    base_url: Url,
    http: HttpClient,
}

impl WarframeClient {
    ///
    /// Create new WarframeClient that can be reused for multiple requests
    ///
    pub fn new() -> Self {
        let base_url = Url::parse(API_URL).expect(&format!("couldn't parse url from {}", API_URL));
        let http = HttpClient::new();
        Self { base_url, http }
    }

    ///
    /// Get the whole data for a `model::Platform` and a choosen `model::Language`
    ///
    pub async fn get_platform(
        &self,
        platform: model::PlatformType,
        lang: model::Language,
    ) -> Result<model::Platform, Box<dyn std::error::Error>> {
        let mut get_url = self.base_url.clone();
        get_url.set_path(platform.into());
        get_url.set_query(Some(&format!("language={}", lang.to_string())));
        let res = self.http.get(get_url).send().await?;
        let platform: model::Platform = res.json().await?;
        Ok(platform)
    }
}

#[derive(Debug)]
pub struct Cache;

#[cfg(test)]
mod tests {
    use crate::model::*;
    use crate::WarframeClient;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    pub fn test_platform_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_json = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_json.push("resources/test/pc_test.json");
        let content = fs::read_to_string(test_json)?;
        let _: Platform = serde_json::from_str(&content)?;
        Ok(())
    }

    #[test]
    pub fn test_platform_lang() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_json = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_json.push("resources/test/pc_test.json");
        let content = fs::read_to_string(test_json)?;
        let pl: Platform = serde_json::from_str(&content)?;
        assert!(pl
            .news
            .iter()
            .any(|news| news.translations.get(Language::English).is_some()));
        Ok(())
    }

    #[test]
    pub fn test_base_url_parsing() {
        WarframeClient::new();
    }
}
