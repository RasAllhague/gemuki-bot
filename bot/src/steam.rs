use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ApplistResponse {
    applist: Applist,
}

impl ApplistResponse {
    pub fn applist(&self) -> &Applist {
        &self.applist
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Applist {
    apps: Vec<App>,
}

impl Applist {
    pub fn apps(&self) -> &[App] {
        &self.apps
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct App {
    appid: u32,
    name: String,
}

impl App {
    pub fn appid(&self) -> u32 {
        self.appid
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GameData {
    pub success: bool,
    pub data: AppDetails,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AppDetails {
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    pub app_type: String,
    pub name: String,
    pub steam_appid: u32,
    pub required_age: u8,
    pub is_free: bool,
    pub controller_support: String,
    pub dlc: Vec<u32>,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub header_image: String,
    pub capsule_image: String,
    pub capsule_imagev5: String,
    pub website: String,
    pub pc_requirements: Requirements,
    pub mac_requirements: Requirements,
    pub linux_requirements: Requirements,
    pub developers: Vec<String>,
    pub publishers: Vec<String>,
    pub price_overview: PriceOverview,
    pub packages: Vec<u32>,
    pub package_groups: Vec<PackageGroup>,
    pub platforms: Platforms,
    pub metacritic: Metacritic,
    pub categories: Vec<Category>,
    pub genres: Vec<Genre>,
    pub screenshots: Vec<Screenshot>,
    pub movies: Vec<Movie>,
    pub recommendations: Recommendations,
    pub achievements: Achievements,
    pub release_date: ReleaseDate,
    pub support_info: SupportInfo,
    pub background: String,
    pub background_raw: String,
    pub content_descriptors: ContentDescriptors,
    pub ratings: Ratings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Requirements {
    pub minimum: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PriceOverview {
    pub currency: String,
    pub initial: u32,
    #[serde(rename(serialize = "final"))]
    #[serde(rename(deserialize = "final"))]
    pub final_number: u32,
    pub discount_percent: u8,
    pub initial_formatted: String,
    pub final_formatted: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PackageGroup {
    pub name: String,
    pub title: String,
    pub description: String,
    pub selection_text: String,
    pub save_text: String,
    pub display_type: u8,
    pub is_recurring_subscription: String,
    pub subs: Vec<Sub>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Sub {
    pub packageid: u32,
    pub percent_savings_text: String,
    pub percent_savings: u8,
    pub option_text: String,
    pub option_description: String,
    pub can_get_free_license: String,
    pub is_free_license: bool,
    pub price_in_cents_with_discount: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Platforms {
    pub windows: bool,
    pub mac: bool,
    pub linux: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Metacritic {
    pub score: u8,
    pub url: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: u8,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Genre {
    pub id: String,
    pub description: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Screenshot {
    pub id: u8,
    pub path_thumbnail: String,
    pub path_full: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Movie {
    pub id: u32,
    pub name: String,
    pub thumbnail: String,
    pub webm: Webm,
    pub mp4: Mp4,
    pub highlight: bool,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Webm {
    pub _480: String,
    pub max: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Mp4 {
    pub _480: String,
    pub max: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Recommendations {
    pub total: u32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Achievements {
    pub total: u8,
    pub highlighted: Vec<Highlighted>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Highlighted {
    pub name: String,
    pub path: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReleaseDate {
    pub coming_soon: bool,
    pub date: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SupportInfo {
    pub url: String,
    pub email: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContentDescriptors {
    pub ids: Vec<u8>,
    pub notes: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Ratings {
    pub dejus: Rating,
    pub steam_germany: Rating,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Rating {
    pub rating_generated: String,
    pub rating: String,
    pub required_age: String,
    pub banned: String,
    pub use_age_gate: String,
    pub descriptors: String,
}

#[derive(Debug)]
pub enum SteamError {
    Reqwest(reqwest::Error),
    Serde(serde_json::Error),
}

impl From<reqwest::Error> for SteamError {
    fn from(value: reqwest::Error) -> Self {
        SteamError::Reqwest(value)
    }
}

impl From<serde_json::Error> for SteamError {
    fn from(value: serde_json::Error) -> Self {
        SteamError::Serde(value)
    }
}

pub async fn get_app_details(appid: u32) -> Result<AppDetails, SteamError> {
    let body = reqwest::get(format!(
        "http://store.steampowered.com/api/appdetails?appids={}&cc=de",
        appid
    ))
    .await?
    .text()
    .await?;

    let appdata: GameData = serde_json::de::from_str(&body)?;

    Ok(appdata.data)
}
