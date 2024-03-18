pub mod model;

const API_URL: &'static str = "api.warframestat.us";
const PROTOCOL: &'static str = "https";

#[derive(Debug)]
pub struct Client;

#[derive(Debug)]
pub struct ClientBuilder;

#[derive(Debug)]
pub struct Cache;

#[cfg(test)]
mod tests {
    use crate::model::*;
    use std::fs;
    use std::path::PathBuf;

    #[test]
    pub fn test_platform_deserialize() -> Result<(), Box<dyn std::error::Error>> {
        let mut test_json = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        test_json.push("resources/test/pc_test.json");
        let content = fs::read_to_string(test_json)?;
        let v: Platform = serde_json::from_str(&content)?;
        Ok(())
    }
}
