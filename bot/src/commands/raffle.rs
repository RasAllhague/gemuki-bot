use gemuki_service::query::raffle_query;

use crate::{paginate, Data, PoiseError};

type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// A command for managing games.
#[poise::command(
    slash_command,
    subcommands("list", "create", "start", "abort", "end", "delete", "add_key")
)]
pub async fn raffle(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

#[poise::command(slash_command)]
pub async fn list(ctx: Context<'_>) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;
    let key_raffles = raffle_query::get_all(db).await?;

    if !key_raffles.is_empty() {
        paginate::create_pagination(ctx, &key_raffles).await?;
    } else {
        ctx.reply("No games found.").await?;
    }

    Ok(())
}

#[poise::command(slash_command)]
pub async fn create(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}

#[poise::command(slash_command, guild_only)]
pub async fn start(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}

#[poise::command(slash_command, guild_only)]
pub async fn abort(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}

#[poise::command(slash_command, guild_only)]
pub async fn end(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}

#[poise::command(slash_command)]
pub async fn delete(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}

#[poise::command(slash_command)]
pub async fn add_key(ctx: Context<'_>,) -> Result<(), PoiseError> {
    todo!()
}