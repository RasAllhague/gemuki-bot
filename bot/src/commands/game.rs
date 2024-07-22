use poise::serenity_prelude::{self as serenity, CreateMessage};

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// A command for managing games.
#[poise::command(
    slash_command,
    owners_only,
    subcommands("list", "details", "add", "edit", "remove")
)]
pub async fn game(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Lists all games currently saved in the key database.
#[poise::command(slash_command, owners_only)]
pub async fn list(_: Context<'_>) -> Result<(), PoiseError> {
    Ok(())
}

/// Displays all details currently available to a game.
#[poise::command(slash_command, owners_only)]
pub async fn details(
    _: Context<'_>,
    #[description = "Id of the game you want to see details of."] id: i32,
) -> Result<(), PoiseError> {
    Ok(())
}

/// Adds a new game.
#[poise::command(slash_command, owners_only)]
pub async fn add(
    _: Context<'_>,
    #[description = "Title of the game you want to add."] title: String,
    #[description = "Description of the game you want to add. Optional."] description: Option<
        String,
    >,
) -> Result<(), PoiseError> {
    Ok(())
}

/// Edits details of a game.
#[poise::command(slash_command, owners_only)]
pub async fn edit(
    _: Context<'_>,
    #[description = "Title of the game you want to edit."] title: Option<String>,
    #[description = "Description of the game you want to edit."] description: Option<String>,
) -> Result<(), PoiseError> {
    Ok(())
}

/// Removes a game entry. Use on own risk as it also clears KEYs connected to the game.
#[poise::command(slash_command, owners_only)]
pub async fn remove(
    _: Context<'_>,
    #[description = "Id of the game you want to delete."] id: i32,
) -> Result<(), PoiseError> {
    Ok(())
}
