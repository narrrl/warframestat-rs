pub mod model;
use model::*;

use chrono::{DateTime, Local};
use reqwest::{Client as HttpClient, Response};
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
    cache: WarframeCache<&'static str>,
}

impl Default for WarframeClient {
    fn default() -> WarframeClient {
        WarframeClient {
            base_url: Url::parse(API_URL).expect(&format!("couldn't parse url from {}", API_URL)),
            ..Default::default()
        }
    }
}

///
/// defines a cache entry to validate cache entries based on the `timestamp`
///
#[derive(Debug, Clone)]
pub struct CacheEntry<T>
where
    T: Clone,
{
    pub entry: T,
    expiration_time: DateTime<Local>,
}

impl<T: Clone> CacheEntry<T> {
    fn is_expired(&self) -> bool {
        Local::now() >= self.expiration_time
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct CacheKey<T>
where
    T: std::hash::Hash + PartialEq + Eq,
{
    pub key_value: T,
    pub language: Language,
    pub platform: PlatformType,
}

impl<T: std::hash::Hash + PartialEq + Eq> CacheKey<T> {
    fn new(key_value: T, language: Language, platform: PlatformType) -> Self {
        Self {
            key_value,
            language,
            platform,
        }
    }
}

impl<T: Clone> CacheEntry<T> {
    fn new(entry: T, expiration_time: DateTime<Local>) -> Self {
        Self {
            entry,
            expiration_time,
        }
    }
}

///
/// Cache to cache the results of `WarframeClient`. This threadsafe because it internally uses an
/// arc mutex
///
#[derive(Debug, Default, Clone)]
pub struct WarframeCache<T: std::hash::Hash + PartialEq + Eq> {
    map: Arc<Mutex<HashMap<CacheKey<T>, CacheEntry<String>>>>,
}

impl<T: std::hash::Hash + PartialEq + Eq> WarframeCache<T> {
    pub fn new() -> Self {
        Self {
            map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    ///
    /// get a json from the cache, returns `None` if not cached
    ///
    pub async fn get(&self, key: T, language: Language, platform: PlatformType) -> Option<String> {
        let locked_map = self.map.lock().await;
        let item = locked_map.get(&CacheKey::new(key, language, platform));
        if let Some(i) = item {
            if i.is_expired() {
                return None;
            }
        }
        item.map(|entry| entry.entry.to_string())
    }

    pub async fn insert(
        &mut self,
        key: T,
        language: Language,
        platform: PlatformType,
        entry: String,
        expiration_time: DateTime<Local>,
    ) {
        let mut map = self.map.lock().await;
        map.insert(
            CacheKey::new(key, language, platform),
            CacheEntry::new(entry, expiration_time),
        );
    }
}

impl WarframeClient {
    ///
    /// Create new WarframeClient that can be reused for multiple requests
    ///
    pub fn new() -> Self {
        Self::default()
    }

    ///
    /// Get the whole data for a `model::Platform` and a choosen `Language`
    ///
    pub async fn get_platform(
        &self,
        platform: PlatformType,
        lang: Language,
    ) -> Result<model::Platform, Box<dyn std::error::Error>> {
        let mut get_url = self.base_url.clone();
        get_url.set_path(platform.into());
        get_url.set_query(Some(&format!("language={}", lang.to_string())));
        let res = self.http.get(get_url).send().await?;
        let platform: model::Platform = res.json().await?;
        Ok(platform)
    }

    async fn request(
        &self,
        path: &str,
        platform: PlatformType,
        lang: Language,
    ) -> Result<Response, reqwest::Error> {
        let mut get_url = self.base_url.clone();
        get_url.set_path((platform.to_string() + path).as_str());
        get_url.set_query(Some(&format!("language={}", lang.to_string())));
        self.http.get(get_url).send().await
    }

    pub async fn get_alerts(
        &mut self,
        platform: PlatformType,
        lang: Language,
    ) -> Result<Vec<Alert>, Box<dyn std::error::Error>> {
        if let Some(s) = self.cache.get("alerts", lang, platform).await {
            return serde_json::from_str(&s)?;
        }
        let res = self.request("/alerts", platform, lang).await?;
        let obj: Vec<Alert> = res.json().await?;
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
