use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use thiserror::Error;

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

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct GameData {
    pub success: bool,
    pub data: AppDetails,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppDetails {
    #[serde(rename(serialize = "type"))]
    #[serde(rename(deserialize = "type"))]
    pub app_type: String,
    pub name: String,
    pub steam_appid: u32,
    pub is_free: bool,
    pub detailed_description: String,
    pub about_the_game: String,
    pub short_description: String,
    pub header_image: String,
    pub website: String,
    pub price_overview: PriceOverview,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PriceOverview {
    pub currency: String,
    pub initial: u32,
    #[serde(rename(serialize = "final"))]
    #[serde(rename(deserialize = "final"))]
    pub final_value: u32,
    pub discount_percent: u8,
    pub initial_formatted: String,
    pub final_formatted: String,
}

#[derive(Error, Debug)]
pub enum SteamError {
    #[error("request to steam failed")]
    Reqwest(#[from] reqwest::Error),
    #[error("failed to parse steam json")]
    Serde(#[from] serde_json::Error),
}

pub async fn get_app_details(appid: u32) -> Result<Option<AppDetails>, SteamError> {
    let body = reqwest::get(format!(
        "http://store.steampowered.com/api/appdetails?appids={}&cc=de",
        appid
    ))
    .await?
    .text()
    .await?
    .replace("\n", "");

    let appdata: HashMap<String, GameData> = serde_json::de::from_str(&body)?;
    let entry = appdata.get(&appid.to_string());

    match entry {
        Some(e) => Ok(Some(e.data.clone())),
        None => Ok(None),
    }
}