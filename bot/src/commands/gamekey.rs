use poise::serenity_prelude::{
    self as serenity,
    futures::{self, Stream, StreamExt},
    CreateMessage,
};

use crate::Data;

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
type Context<'a> = poise::Context<'a, Data, PoiseError>;

#[derive(Debug, poise::ChoiceParameter)]
pub enum KeystateCoice {
    #[name = "Unused"]
    Unused,
    #[name = "Used"]
    Used,
}

#[derive(Debug, poise::ChoiceParameter)]
pub enum PlatformCoice {
    #[name = "Steam"]
    Steam,
    #[name = "Epic Games"]
    Epic,
    #[name = "Ubisoft Connect"]
    UPlay,
    #[name = "EA Play"]
    EA,
}

async fn autocomplete_game<'a>(
    _ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    futures::stream::iter(&["Amanda", "Bob", "Christian", "Danny", "Ester", "Falk"])
        .filter(move |name| futures::future::ready(name.starts_with(partial)))
        .map(|name| name.to_string())
}

/// A command for managing games.
#[poise::command(
    slash_command,
    owners_only,
    subcommands("list", "details", "add", "remove", "edit", "claim")
)]
pub async fn gamekey(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Lists all keys of a game. Contains severel filter options.
#[poise::command(slash_command, owners_only)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Id of the game you want to list keys from."] game_id: i32,
    #[description = "Filter for state of keys."] keystate: Option<KeystateCoice>,
    #[description = "Filter for the platform."] platform: Option<PlatformCoice>,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Displays the details of a gamekey except its key.
#[poise::command(slash_command, owners_only)]
pub async fn details(
    ctx: Context<'_>,
    #[description = "Id of the gamekey."] gamekey_id: i32,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Adds a gamekey for to a game.
#[poise::command(slash_command, owners_only, dm_only)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: String,
    #[description = "Platform of the game key."] platform: PlatformCoice,
    #[description = "State of the key."] keystate: KeystateCoice,
    #[description = "Value of the key."] value: String,
    #[description = "Store page link of the key."] page_link: Option<String>,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Removes a gamekey from a game. Use at own risk as the key gets deleted.
#[poise::command(slash_command, owners_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Id of the gamekey to delete"] gamekey_id: i32,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Edits the details of a game key.
#[poise::command(slash_command, owners_only)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: Option<String>,
    #[description = "Platform of the game key."] platform: Option<PlatformCoice>,
    #[description = "State of the key."] keystate: Option<KeystateCoice>,
    #[description = "Value of the key."] value: Option<String>,
    #[description = "Store page link of the key."] page_link: Option<String>,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Claims a key. Sends the key value hidden behind a spoiler into the channel.
#[poise::command(slash_command, owners_only)]
pub async fn claim(
    ctx: Context<'_>,
    #[description = "Id of the key you want to claim."] gamekey_id: i32,
) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}
