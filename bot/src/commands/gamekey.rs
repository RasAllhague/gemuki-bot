use chrono::Utc;
use entity::game_key;
use gemuki_service::{
    mutation::GameKeyMutation,
    query::{GameKeyQuery, GameQuery, PlatformQuery},
};
use log::{error, warn};
use poise::{
    serenity_prelude::{
        futures::{self, Stream, StreamExt},
        CreateEmbed, Embed,
    },
    CreateReply,
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

impl ToString for KeystateCoice {
    fn to_string(&self) -> String {
        match self {
            KeystateCoice::Unused => "Unused".to_owned(),
            KeystateCoice::Used => "Used".to_owned(),
        }
    }
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

impl ToString for PlatformCoice {
    fn to_string(&self) -> String {
        match self {
            PlatformCoice::Steam => "Steam".to_owned(),
            PlatformCoice::Epic => "Epic Games".to_owned(),
            PlatformCoice::UPlay => "Ubisoft Connect".to_owned(),
            PlatformCoice::EA => "EA Play".to_owned(),
        }
    }
}

async fn autocomplete_game<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let db = &ctx.data().conn;

    let games = match GameQuery::get_all(db).await {
        Ok(g) => g.iter().map(|x| x.title.clone()).collect(),
        Err(_) => Vec::new(),
    };

    futures::stream::iter(games)
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
    let db = &ctx.data().conn;

    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Displays the details of a gamekey except its key.
#[poise::command(slash_command, owners_only)]
pub async fn details(
    ctx: Context<'_>,
    #[description = "Id of the gamekey."] gamekey_id: i32,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let game_key = match GameKeyQuery::get_one(db, gamekey_id).await? {
        Some(g) => g,
        None => {
            ctx.say(format!("The gamekey `{}` does not exist.", gamekey_id))
                .await?;
            return Ok(());
        }
    };
    let game = match GameQuery::get_one(db, game_key.game_id).await? {
        Some(g) => g,
        None => {
            ctx.say(format!("The game `{}` does not exist.", game_key.game_id))
                .await?;
            return Ok(());
        }
    };
    let platform = match PlatformQuery::get_one(db, game_key.platform_id).await? {
        Some(g) => g,
        None => {
            ctx.say(format!(
                "The platform `{}` does not exist.",
                game_key.platform_id
            ))
            .await?;
            return Ok(());
        }
    };

    let embed = CreateEmbed::new()
        .title(format!("{} Key-Id {}", game.title, game_key.id))
        .description(game.description.unwrap_or("None".to_owned()))
        .field("Platform", platform.name, true)
        .field("State", game_key.keystate, true)
        .field("Create date", game_key.create_date, false)
        .field("Create user id", game_key.create_user_id.to_string(), false)
        .field(
            "Modify date",
            game_key.modify_date.unwrap_or("None".to_owned()),
            false,
        )
        .field(
            "Modify user id",
            game_key.modify_user_id.unwrap_or(0).to_string(),
            false,
        );

    let embed = match game_key.page_link {
        Some(link) => embed.url(link),
        None => embed,
    };
    let embed = match game.image_link {
        Some(link) => embed.image(link),
        None => embed,
    };

    ctx.send(CreateReply::default().embed(embed)).await?;

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
    let db = &ctx.data().conn;

    let game = match GameQuery::get_by_title(db, &game).await? {
        Some(g) => g,
        None => {
            ctx.send(
                CreateReply::default()
                    .content(format!("The game `{}` does not exist.", game))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };
    let platform = match PlatformQuery::get_by_name(db, &platform.to_string()).await? {
        Some(g) => g,
        None => {
            ctx.send(
                CreateReply::default()
                    .content(format!(
                        "The platform `{}` does not exist.",
                        platform.to_string()
                    ))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    let model = game_key::Model {
        id: 0,
        game_id: game.id,
        platform_id: platform.id,
        value,
        keystate: keystate.to_string(),
        page_link: page_link,
        create_date: Utc::now().naive_utc().to_string(),
        create_user_id: ctx.author().id.into(),
        modify_date: None,
        modify_user_id: None,
    };

    let message = match GameKeyMutation::create(db, model).await {
        Ok(_) => "Successfully added key.",
        Err(why) => {
            error!("Could not insert new game because of '{}'.", why);
            "Could not add game because of an internal server error."
        }
    };

    ctx.send(CreateReply::default().content(message).ephemeral(true))
        .await?;

    Ok(())
}

/// Removes a gamekey from a game. Use at own risk as the key gets deleted.
#[poise::command(slash_command, owners_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Id of the gamekey to delete"] gamekey_id: i32,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let deleted_keys = GameKeyMutation::delete(db, gamekey_id).await?;

    ctx.reply(format!("Deleted `{}` keys.", deleted_keys.rows_affected))
        .await?;

    warn!("Deleted game with id '{gamekey_id}'.");

    Ok(())
}

/// Edits the details of a game key.
#[poise::command(slash_command, owners_only)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Id of the key to edit."] id: i32,
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
    let db = &ctx.data().conn;

    let mut game_key = match GameKeyQuery::get_one(db, gamekey_id).await? {
        Some(g) => g,
        None => {
            ctx.send(
                CreateReply::default()
                    .content(format!("The key `{}` does not exist.", gamekey_id))
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    if game_key.keystate == "Used" {
        ctx.send(
            CreateReply::default()
                .content(format!("The key `{}` is already used.", gamekey_id))
                .ephemeral(true),
        )
        .await?;
        return Ok(());
    }

    let reply = CreateReply::default()
        .content(format!("Your key: `{}`", game_key.value))
        .ephemeral(true);

    game_key.keystate = "Used".to_owned();
    game_key.modify_date = Some(Utc::now().naive_utc().to_string());
    game_key.modify_user_id = Some(ctx.author().id.into());

    GameKeyMutation::update(db, game_key).await?;

    ctx.send(reply).await?;
    Ok(())
}
