pub mod commands;
mod paginate;
pub mod cache;

use async_mutex::Mutex;
use cache::Cache;
use chrono::Duration;
use commands::{game::game, gamekey::gamekey, version::version};
use migration::sea_orm::DatabaseConnection;
use migration::{sea_orm::Database, Migrator, MigratorTrait};
use poise::serenity_prelude::{self as serenity};

pub type PoiseError = Box<dyn std::error::Error + Send + Sync>;

pub struct Data {
    conn: DatabaseConnection,
    cache: Mutex<Cache>,
}

#[tokio::main]
async fn run() -> Result<(), PoiseError> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let token = std::env::var("GEMUKI_TOKEN").expect("Missing GEMUKI_TOKEN.");
    let db_url = std::env::var("GEMUKI_DATABASE_URL").expect("GEMUKI_DATABASE_URL is not set in .env file");
    let intents =
        serenity::GatewayIntents::non_privileged();

    let conn = Database::connect(&db_url).await?;
    Migrator::up(&conn, None).await?;

    //let cache = Cache::init(&conn, Duration::seconds(3600)).await;
    let cache = Cache::init(&conn, Duration::seconds(10)).await;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![version(), game(), gamekey()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data { conn, cache: Mutex::new(cache) })
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client?.start().await?;

    Ok(())
}

pub fn main() {
    let result = run();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
