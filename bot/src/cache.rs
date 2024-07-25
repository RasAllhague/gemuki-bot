use chrono::{Duration, NaiveDateTime, Utc};
use gemuki_service::query::GameQuery;
use log::info;
use migration::sea_orm::DbConn;

pub struct Cache {
    refresh_interval: Duration,
    last_refresh: NaiveDateTime,
    cache: Vec<String>,
}

impl Cache {
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
            Err(_) => Vec::new(),
        }
    }

    pub fn last_refresh(&self) -> NaiveDateTime {
        self.last_refresh
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
