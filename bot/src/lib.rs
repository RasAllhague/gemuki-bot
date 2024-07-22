pub mod commands;

use commands::{game::game, gamekey::gamekey, version::version};
use poise::serenity_prelude::{self as serenity};

type PoiseError = Box<dyn std::error::Error + Send + Sync>;

pub struct Data {}

#[tokio::main]
async fn run() -> Result<(), PoiseError> {
    tracing_subscriber::fmt::init();

    dotenvy::dotenv().ok();
    let token = std::env::var("GEMUKI_TOKEN").expect("Missing GEMUKI_TOKEN.");
    let intents =
        serenity::GatewayIntents::non_privileged() | serenity::GatewayIntents::GUILD_MEMBERS;

    let framework = poise::Framework::builder()
        .options(poise::FrameworkOptions {
            commands: vec![version(), game(), gamekey()],
            ..Default::default()
        })
        .setup(|ctx, _ready, framework| {
            Box::pin(async move {
                poise::builtins::register_globally(ctx, &framework.options().commands).await?;
                Ok(Data {})
            })
        })
        .build();

    let client = serenity::ClientBuilder::new(token, intents)
        .framework(framework)
        .await;
    client.unwrap().start().await.unwrap();

    Ok(())
}

pub fn main() {
    let result = run();

    if let Some(err) = result.err() {
        println!("Error: {err}");
    }
}
