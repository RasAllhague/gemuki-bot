use gemuki_service::query::{GameKeyQuery, GameQuery};
use poise::{serenity_prelude::CreateEmbed, CreateReply};

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// Displays some statistics for the bot.
#[poise::command(slash_command)]
pub async fn statistics(ctx: Context<'_>) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let total_games = GameQuery::count_total(db).await?;
    let total_keys = GameKeyQuery::count_total(db).await?;
    let unused_keys = GameKeyQuery::count_unused(db).await?;
    let used_keys = GameKeyQuery::count_used(db).await?;

    let embed = CreateEmbed::new()
        .title("gemuki-bot statistics")
        .field("Total games", total_games.to_string(), true)
        .field("Total keys", total_keys.to_string(), true)
        .field("Unused keys", unused_keys.to_string(), true)
        .field("Used keys", used_keys.to_string(), true);

    ctx.send(CreateReply::default().embed(embed)).await?;

    Ok(())
}
