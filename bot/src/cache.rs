use chrono::{Duration, NaiveDateTime, Utc};
use gemuki_service::query::GameQuery;
use log::{error, info};
use migration::sea_orm::DbConn;

use crate::steam::{App, ApplistResponse};

static GET_ALL_APPS_URL: &str =
    "https://api.steampowered.com/ISteamApps/GetAppList/v0002/?format=json";

pub struct GameTitleCache {
    refresh_interval: Duration,
    last_refresh: NaiveDateTime,
    cache: Vec<String>,
}

impl GameTitleCache {
    pub async fn init(db: &DbConn, refresh_interval: Duration) -> Self {
        let titles = Self::get_game_titles(db).await;

        Self {
            refresh_interval,
            last_refresh: Utc::now().naive_utc(),
            cache: titles,
        }
    }

    async fn get_game_titles(db: &DbConn) -> Vec<String> {
        match GameQuery::get_all(db).await {
            Ok(g) => g.iter().map(|x| x.title.clone()).collect(),
            Err(why) => {
                error!("An error occured while trying to get titles: {why}");
                Vec::new()
            }
        }
    }

    pub fn cache(&self) -> &[String] {
        &self.cache
    }

    pub async fn update(&mut self, db: &DbConn) {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let delta = now - self.last_refresh;

        if delta.num_seconds() > self.refresh_interval.num_seconds() || self.cache.is_empty() {
            self.cache = Self::get_game_titles(db).await;
            self.last_refresh = now;

            info!("Cache has been updated.");
        }
    }

    pub async fn force_update(&mut self, db: &DbConn) {
        self.cache = Self::get_game_titles(db).await;
        self.last_refresh = Utc::now().naive_utc();

        info!("Cache has been updated forcefully.");
    }
}

pub struct SteamAppCache {
    refresh_interval: Duration,
    last_refresh: NaiveDateTime,
    cache: Vec<App>,
}

impl SteamAppCache {
    pub async fn init(refresh_interval: Duration) -> Self {
        let titles = Self::get_apps().await;

        Self {
            refresh_interval,
            last_refresh: Utc::now().naive_utc(),
            cache: titles,
        }
    }

    async fn get_apps() -> Vec<App> {
        let body = match Self::request_apps().await {
            Ok(b) => b,
            Err(why) => {
                error!("An error occured while trying to get all steam apps: {why}");
                return Vec::new();
            }
        };

        let apps_response = match serde_json::de::from_str::<ApplistResponse>(&body) {
            Ok(r) => r,
            Err(why) => {
                error!("An error occured while trying to parse apps response: {why}");
                return Vec::new();
            }
        };

        let apps: Vec<App> = apps_response
            .applist()
            .apps()
            .iter()
            .filter(|x| !x.name().trim().is_empty())
            .map(|x| x.clone())
            .collect();

        apps
    }

    async fn request_apps() -> Result<String, reqwest::Error> {
        reqwest::get(GET_ALL_APPS_URL).await?.text().await
    }

    pub fn find_by_name(&self, name: &str) -> Option<&App> {
        self.cache.iter().find(|x| x.name() == name)
    }

    pub async fn update(&mut self) {
        let now: NaiveDateTime = Utc::now().naive_utc();
        let delta = now - self.last_refresh;

        if delta.num_seconds() > self.refresh_interval.num_seconds() || self.cache.is_empty() {
            self.cache = Self::get_apps().await;
            self.last_refresh = now;

            info!("Cache has been updated.");
        }
    }
}
