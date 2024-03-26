use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::time::SystemTime;

/// Enum to represent the different platforms
///
/// Defaults to "Platform::PC"
#[derive(PartialEq, Eq, Debug, Hash)]
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
#[derive(PartialEq, Eq, Debug, Hash)]
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

trait Expirable {
    fn expiry_time() -> SystemTime;
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Platform {
    pub timestamp: String,
    pub news: Vec<News>,
    pub events: Vec<Event>,
    pub alerts: Vec<Alert>,
    pub sortie: Sortie,
    pub syndicate_missions: Vec<SyndicateMission>,
    pub fissures: Vec<Fissure>,
    pub flash_sales: Vec<FlashSale>,
    pub invasions: Vec<Invasion>,
    pub void_traders: Vec<VoidTrader>,
    pub void_trader: VoidTrader,
    pub daily_deals: Vec<DailyDeal>,
    pub simaris: Simaris,
    pub conclave_challenges: Vec<ConclaveChallenge>,
    pub earth_cycle: EarthCycle,
    pub cetus_cycle: CetusCycle,
    pub cambion_cycle: CambionCycle,
    pub zariman_cycle: ZarimanCycle,
    pub construction_progress: ConstructionProgress,
    pub vallis_cycle: VallisCycle,
    pub nightwave: Nightwave,
    pub kuva: Vec<Kuva>,
    pub arbitration: Arbitration,
    pub sentient_outposts: SentientOutposts,
    pub steel_path: SteelPath,
    pub vault_trader: VaultTrader,
    pub archon_hunt: ArchonHunt,
    pub duviri_cycle: DuviriCycle,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct News {
    pub id: String,
    pub message: String,
    pub link: String,
    pub image_link: String,
    pub priority: bool,
    pub date: String,
    pub eta: String,
    pub update: bool,
    pub prime_access: bool,
    pub stream: bool,
    pub translations: Translations,
    pub as_string: String,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Translations(HashMap<String, String>);

impl Translations {
    pub fn get(&self, lang: Language) -> Option<&String> {
        self.0.get(lang.into())
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub maximum_score: i64,
    pub current_score: i64,
    pub small_interval: Option<i64>,
    pub large_interval: Option<i64>,
    pub description: String,
    pub tooltip: String,
    pub node: String,
    pub concurrent_nodes: Vec<String>,
    pub score_loc_tag: String,
    pub rewards: Vec<Reward>,
    pub expired: bool,
    pub health: i64,
    pub interim_steps: Vec<InterimStep>,
    pub progress_steps: Vec<ProgressStep>,
    pub is_personal: bool,
    pub is_community: bool,
    pub region_drops: Vec<String>,
    pub archwing_drops: Vec<String>,
    pub as_string: String,
    pub completion_bonuses: Vec<i64>,
    pub score_var: String,
    pub alt_expiry: String,
    pub alt_activation: String,
    pub next_alt: NextAlt,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStep {
    #[serde(rename = "type")]
    pub step_type: String,
    progress_amt: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Reward {
    pub items: Vec<String>,
    pub counted_items: Vec<CountedItem>,
    pub credits: i64,
    pub as_string: String,
    pub item_string: String,
    pub thumbnail: String,
    pub color: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InterimStep {
    pub goal: i64,
    pub reward: Reward,
    pub message: Message,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NextAlt {
    pub expiry: String,
    pub activation: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Alert {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub mission: Mission,
    pub eta: String,
    pub reward_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mission {
    pub node: String,
    pub node_key: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type_key: String,
    pub faction: String,
    pub faction_key: String,
    pub reward: Reward,
    pub min_enemy_level: i64,
    pub max_enemy_level: i64,
    pub nightmare: bool,
    pub archwing_required: bool,
    pub is_sharkwing: bool,
    pub level_override: String,
    pub enemy_spec: String,
    pub advanced_spawners: Vec<String>,
    pub required_items: Vec<String>,
    pub level_auras: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Sortie {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub reward_pool: String,
    pub variants: Vec<Variant>,
    pub missions: Vec<Mission>,
    pub boss: String,
    pub faction: String,
    pub faction_key: String,
    pub expired: bool,
    pub eta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Variant {
    pub mission_type: String,
    pub mission_type_key: String,
    pub modifier: String,
    pub modifier_description: String,
    pub node: String,
    pub node_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SyndicateMission {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub syndicate: String,
    pub syndicate_key: String,
    pub nodes: Vec<String>,
    pub jobs: Vec<Job>,
    pub eta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    pub id: String,
    pub reward_pool: Vec<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub enemy_levels: Vec<i64>,
    pub standing_stages: Vec<i64>,
    #[serde(rename = "minMR")]
    pub min_mr: u8,
    pub expiry: String,
    pub time_bound: Option<String>,
    pub is_vault: Option<bool>,
    pub location_tag: Option<String>,
    pub time_boound: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Fissure {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub node: String,
    pub mission_type: String,
    pub mission_key: String,
    pub enemy: String,
    pub enemy_key: String,
    pub node_key: String,
    pub tier: String,
    pub tier_num: u8,
    pub expired: bool,
    pub eta: String,
    pub is_storm: bool,
    pub is_hard: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FlashSale {
    pub item: String,
    pub expiry: String,
    pub activation: String,
    pub discount: i64,
    pub regular_override: Option<i64>,
    pub premium_override: i64,
    pub is_shown_in_market: bool,
    pub id: String,
    pub expired: bool,
    pub eta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Invasion {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub node: String,
    pub node_key: String,
    pub desc: String,
    pub attacking_faction: String,
    pub attacker: Attacker,
    pub defender: Defender,
    pub vs_infestation: bool,
    pub count: i64,
    pub required_runs: i64,
    pub completion: f64,
    pub completed: bool,
    pub eta: String,
    pub reward_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Attacker {
    pub reward: Option<Reward>,
    pub faction: String,
    pub faction_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CountedItem {
    pub count: i64,
    #[serde(rename = "type")]
    pub type_field: String,
    pub key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Defender {
    pub reward: Reward,
    pub faction: String,
    pub faction_key: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidTrader {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub character: String,
    pub location: String,
    pub inventory: Vec<VoidItem>,
    pub ps_id: String,
    pub end_string: String,
    pub initial_start: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VoidItem {
    pub item: String,
    pub ducats: i64,
    pub credits: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyDeal {
    pub item: String,
    pub unique_name: String,
    pub expiry: String,
    pub activation: String,
    pub original_price: i64,
    pub sale_price: i64,
    pub total: i64,
    pub sold: i64,
    pub id: String,
    pub eta: String,
    pub discount: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Simaris {
    pub target: String,
    pub is_target_active: bool,
    pub as_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConclaveChallenge {
    pub id: String,
    pub expiry: String,
    pub activation: String,
    pub amount: i64,
    pub mode: String,
    pub category: String,
    pub eta: String,
    pub expired: bool,
    pub daily: bool,
    pub root_challenge: bool,
    pub end_string: String,
    pub description: String,
    pub title: String,
    pub standing: i64,
    pub as_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EarthCycle {
    pub id: String,
    pub expiry: String,
    pub activation: String,
    pub is_day: bool,
    pub state: String,
    pub time_left: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CetusCycle {
    pub id: String,
    pub expiry: String,
    pub activation: String,
    pub is_day: bool,
    pub state: String,
    pub time_left: String,
    pub is_cetus: bool,
    pub short_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CambionCycle {
    pub id: String,
    pub activation: String,
    pub expiry: String,
    pub time_left: String,
    pub state: String,
    pub active: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZarimanCycle {
    pub id: String,
    pub expiry: String,
    pub activation: String,
    pub is_corpus: bool,
    pub state: String,
    pub time_left: String,
    pub short_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstructionProgress {
    pub id: String,
    pub fomorian_progress: String,
    pub razorback_progress: String,
    pub unknown_progress: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VallisCycle {
    pub id: String,
    pub expiry: String,
    pub is_warm: bool,
    pub state: String,
    pub activation: String,
    pub time_left: String,
    pub short_string: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Nightwave {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub season: i64,
    pub tag: String,
    pub phase: i64,
    pub params: Params,
    pub active_challenges: Vec<ActiveChallenge>,
    pub reward_types: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Params {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActiveChallenge {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub is_daily: bool,
    pub is_elite: bool,
    pub desc: String,
    pub title: String,
    pub reputation: i64,
    pub is_permanent: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Arbitration {
    pub activation: String,
    pub expiry: String,
    pub enemy: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub archwing: bool,
    pub sharkwing: bool,
    pub node: String,
    pub node_key: String,
    pub type_key: String,
    pub id: String,
    pub expired: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentientOutposts {
    pub mission: SentientMission,
    pub activation: String,
    pub expiry: String,
    pub active: bool,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SentientMission {
    pub node: String,
    pub faction: String,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SteelPath {
    pub current_reward: CurrentReward,
    pub activation: String,
    pub expiry: String,
    pub remaining: String,
    pub rotation: Vec<Rotation>,
    pub evergreens: Vec<Evergreen>,
    pub incursions: Incursions,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CurrentReward {
    pub name: String,
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rotation {
    pub name: String,
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Evergreen {
    pub name: String,
    pub cost: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Incursions {
    pub id: String,
    pub activation: String,
    pub expiry: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct VaultTrader {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub character: String,
    pub location: String,
    pub inventory: Vec<Inventory>,
    pub ps_id: String,
    pub end_string: String,
    pub initial_start: String,
    pub completed: bool,
    pub schedule: Vec<Schedule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Inventory {
    pub unique_name: String,
    pub item: String,
    pub ducats: Option<i64>,
    pub credits: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Schedule {
    pub expiry: String,
    pub item: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchonHunt {
    pub id: String,
    pub activation: String,
    pub start_string: String,
    pub expiry: String,
    pub active: bool,
    pub reward_pool: String,
    pub missions: Vec<ArchonHuntMission>,
    pub boss: String,
    pub faction: String,
    pub faction_key: String,
    pub expired: bool,
    pub eta: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ArchonHuntMission {
    pub node: String,
    pub node_key: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub type_key: String,
    pub nightmare: bool,
    pub archwing_required: bool,
    pub is_sharkwing: bool,
    pub advanced_spawners: Vec<String>,
    pub required_items: Vec<String>,
    pub level_auras: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DuviriCycle {
    pub id: String,
    pub activation: String,
    pub expiry: String,
    pub state: String,
    pub choices: Vec<Choice>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Choice {
    pub category: String,
    pub category_key: String,
    pub choices: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Kuva {
    pub id: String,
    pub activation: String,
    pub expiry: String,
    pub start_string: String,
    pub active: bool,
    pub node: String,
    pub enemy: String,
    pub enemy_key: String,
    pub kuva_type: String,
    pub type_key: String,
    pub archwing: bool,
    pub sharkwing: bool,
}
