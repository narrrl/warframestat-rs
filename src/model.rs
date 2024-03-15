use serde::{Deserialize, Serialize};

/// Enum to represent the different platforms
///
/// Defaults to "Platform::PC"
#[derive(PartialEq, Eq, Debug)]
pub enum PlatformType {
    PC,
    PS4,
    XBox,
    Switch,
}

impl Default for PlatformType {
    fn default() -> Self {
        PlatformType::PC
    }
}

impl std::string::ToString for PlatformType {
    fn to_string(&self) -> String {
        let into: &str = self.into();
        into.to_string()
    }
}

impl Into<&str> for &PlatformType {
    fn into(self) -> &'static str {
        match self {
            PlatformType::PC => "pc",
            PlatformType::PS4 => "ps4",
            PlatformType::XBox => "xb1",
            PlatformType::Switch => "swi",
        }
    }
}

impl Into<&str> for PlatformType {
    fn into(self) -> &'static str {
        (&self).into()
    }
}

/// Enum representing the different languages
///
/// Defaults to `Language::English`
#[derive(PartialEq, Eq, Debug)]
pub enum Language {
    English,
    German,
    Spanish,
    French,
    Italian,
    Korean,
    Polish,
    Portuguese,
    Russian,
    Chinese,
    Ukrainian,
}

impl Into<&str> for &Language {
    fn into(self) -> &'static str {
        match self {
            Language::English => "en",
            Language::German => "de",
            Language::Spanish => "es",
            Language::French => "fr",
            Language::Italian => "it",
            Language::Korean => "ko",
            Language::Polish => "pl",
            Language::Portuguese => "pt",
            Language::Russian => "ru",
            Language::Chinese => "zh",
            Language::Ukrainian => "uk",
        }
    }
}

impl Into<&str> for Language {
    fn into(self) -> &'static str {
        (&self).into()
    }
}

impl std::string::ToString for Language {
    fn to_string(&self) -> String {
        let into: &str = self.into();
        into.to_string()
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::English
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    timestamp: String,
    alerts: Vec<Alert>,
    arbitration: Arbitration,
    archon_hunt: ArchonHunt,
    cambion_cycle: CambionDrift,
    cetus_cycle: Cetus,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    mission: Mission,
    expired: bool,
    eta: String,
    reward_types: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    reward: Reward,
    node: String,
    node_key: String,
    faction: String,
    faction_key: String,
    max_enemy_level: u16,
    min_enemy_level: u16,
    max_wave_num: u16,
    #[serde(rename = "type")]
    mission_type: String,
    type_key: String,
    nightmare: bool,
    archwing_required: bool,
    is_sharkwing: bool,
    enemy_spec: String,
    level_override: String,
    advanced_spawners: Vec<String>,
    required_items: Vec<String>,
    consume_required_items: bool,
    leaders_always_allowed: bool,
    level_auras: Vec<String>,
    description: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    counted_items: Vec<ItemBundle>,
    thumbnail: String,
    color: u32,
    credits: u32,
    as_string: String,
    items: Vec<Item>,
    item_string: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Item(String);

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemBundle {
    #[serde(rename = "type")]
    item: Item,
    count: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Arbitration {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    node: String,
    enemy: String,
    enemy_key: String,
    #[serde(rename = "type")]
    arbitration_type: String,
    type_key: String,
    archwing: bool,
    sharkwing: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchonHunt {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    reward_pool: String,
    missions: [Mission; 3],
    boss: String,
    faction: String,
    faction_key: String,
    expired: bool,
    eta: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct CambionDrift {
    id: String,
    expiry: String,
    activation: String,
    state: String,
    active: String,
    time_left: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cetus {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    is_day: bool,
    state: String,
    time_left: String,
    is_cetus: bool,
    short_string: String,
}
