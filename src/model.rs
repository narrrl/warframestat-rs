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
    alerts: Alerts,
    arbitration: Arbitration,
    archon_hunt: ArchonHunt,
    cambion_cycle: CambionDrift,
    cetus_cycle: Cetus,
    conclave_challenges: ConclaveChallenges,
    construction_progress: ConstructionProgress,
    daily_deals: DailyDarvoDeals,
    earth_cycle: EarthRotation,
    events: Events,
    fissures: Fissures,
    flash_sales: DarvoFlashSales,
    global_upgrades: GlobalUpgrades,
}

pub type Alerts = Vec<Alert>;

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

pub type ConclaveChallenges = Vec<ConclaveChallengeData>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConclaveChallengeData {
    mode: String,
    amount: u32,
    eta: String,
    expired: bool,
    end_string: String,
    daily: bool,
    description: String,
    id: String,
    expiry: String,
    as_string: String,
    category: String,
    root_challenge: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ConstructionProgress {
    id: String,
    fomorian_progress: String,
    razorback_progress: String,
    unknown_progress: String,
}

pub type DailyDarvoDeals = Vec<DailyDarvoDeal>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DailyDarvoDeal {
    sold: u32,
    item: String,
    unique_name: String,
    total: u32,
    eta: String,
    original_price: u32,
    sale_price: u32,
    discount: u32,
    expiry: String,
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct EarthRotation {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    is_day: bool,
    time_left: String,
}

pub type Events = Vec<Event>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Event {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    maximum_score: u32,
    current_score: u32,
    small_interval: u32,
    large_interval: u32,
    faction: String,
    descripion: String,
    tooltip: String,
    node: String,
    concurrent_nodes: Vec<String>,
    victim_node: String,
    score_loc_tag: String,
    rewards: Vec<Reward>,
    health: u32,
    affiliated_with: String,
    jobs: Vec<Job>,
    progress_steps: Vec<ProgressStep>,
    progress_total: u32,
    show_total_at_end_of_mission: bool,
    is_personal: bool,
    is_community: bool,
    region_drops: Vec<String>,
    archwing_drops: Vec<String>,
    as_string: String,
    completion_bonuses: Vec<u32>,
    score_var: String,
    alt_expiry: String,
    alt_activation: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProgressStep {
    #[serde(rename = "type")]
    progress_type: String,
    progress_amt: u32,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Job {
    activation: String,
    expiry: String,
    reward_pool: Vec<String>,
    #[serde(rename = "type")]
    job_type: String,
    enemy_levels: Vec<u32>,
    standing_stages: Vec<u32>,
    #[serde(rename = "minMR")]
    min_mr: u8,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct InterimStep {
    goal: u32,
    reward: Reward,
    message: Message,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    sender: String,
    subject: String,
    message: String,
    sender_icon: String,
    attachments: String,
    winner_count: u32,
}

pub type Fissures = Vec<Fissure>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Fissure {
    id: String,
    activation: String,
    expiry: String,
    start_string: String,
    active: bool,
    node: String,
    expired: bool,
    eta: String,
    mission_type: String,
    mission_key: String,
    tier: FissureTier,
    enemy: String,
    is_storm: bool,
    is_hard: bool,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum FissureTier {
    Lith,
    Meso,
    Neo,
    Axi,
    Requiem,
}

pub type DarvoFlashSales = Vec<DarvoFlashSale>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DarvoFlashSale {
    item: String,
    expired: bool,
    eta: String,
    discount: u32,
    premium_override: u32,
    is_popular: bool,
    is_featured: bool,
}

pub type GlobalUpgrades = Vec<GlobalUpgrade>;

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GlobalUpgrade {
    start: String,
    end: String,
    upgrade: String,
    operation: String,
    operation_symbol: String,
    upgrade_operation_value: u32,
    expired: bool,
    eta: String,
    #[serde(rename = "desc")]
    descripion: String,
}
