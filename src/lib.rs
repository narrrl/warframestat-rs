pub mod model;

use reqwest::Client as HttpClient;
use std::collections::HashMap;
use url::Url;

use std::sync::Arc;
use tokio::sync::Mutex;

const API_URL: &'static str = "https://api-warframestat.us";

///
/// WarframeClient to request data from the [Warframestat API](https://doc.warframestat.us)
///
#[derive(Debug, Clone)]
pub struct WarframeClient {
    base_url: Url,
    http: HttpClient,
}

impl Default for WarframeClient {
    fn default() -> WarframeClient {
        WarframeClient {
            base_url: Url::parse(API_URL).expect(&format!("couldn't parse url from {}", API_URL)),
            http: HttpClient::default(),
        }
    }
}

///
/// Cache to cache the results of `WarframeClient`. It is threadsafe because it internally uses an
/// arc mutex
///
#[derive(Debug, Default, Clone)]
pub struct WarframeCache {
    map: Arc<Mutex<HashMap<(&'static str, model::PlatformType, model::Language), String>>>,
}

type CacheResult = Result<String, Box<dyn std::error::Error>>;

impl WarframeCache {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    ///
    /// get a json from the cache, returns `None` if not cached
    ///
    pub async fn get(
        &self,
        key: &str,
        language: model::Language,
        platform: model::PlatformType,
    ) -> Option<String> {
        let lock = self.map.lock().await;
        lock.get(&(key, platform, language))
            .map(|json| json.to_string())
    }

    ///
    /// return a json from the cache or execute a future when the entry doesn't exists and put it
    /// in the cache
    ///
    pub async fn get_or_insert(
        &mut self,
        key: &'static str,
        language: model::Language,
        platform: model::PlatformType,
        future: impl std::future::Future<Output = CacheResult>,
    ) -> CacheResult {
        let mut lock = self.map.lock().await;
        Ok(lock
            .entry((key, platform, language))
            .or_insert(future.await?)
            .to_string())
    }
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
