use chrono::Utc;
use entity::game_key;
use gemuki_service::{
    mutation::GameKeyMutation,
    query::{GameKeyModel, GameKeyQuery, GameQuery, PlatformQuery},
};
use log::{error, warn};
use poise::{serenity_prelude::CreateEmbed, CreateReply};
use rand::Rng;

use crate::{commands::autocomplete_game, paginate, Data};

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

/// A command for managing games.
#[poise::command(
    slash_command,
    subcommands("list", "details", "add", "remove", "edit", "claim", "claim_random", "quickclaim")
)]
pub async fn gamekey(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Lists all keys of a game. Contains severel filter options.
#[poise::command(slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Name of the game you want to see keys of."]
    #[autocomplete = "autocomplete_game"]
    game: String,
    #[description = "Filter for state of keys."] keystate: Option<KeystateCoice>,
    #[description = "Filter for the platform."] platform: Option<PlatformCoice>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let game = match GameQuery::get_by_title(db, &game).await? {
        Some(g) => g,
        None => {
            ctx.reply("Could not find game.").await?;
            return Ok(());
        }
    };
    let game_keys = GameKeyQuery::get_all_by_game(db, game.id, ctx.author().id.get()).await?;

    let game_keys = match keystate {
        Some(choice) => game_keys
            .iter()
            .filter(|x| x.game_key().keystate == choice.to_string())
            .map(|x| x.clone())
            .collect::<Vec<GameKeyModel>>(),
        None => game_keys,
    };

    let game_keys = match platform {
        Some(choice) => game_keys
            .iter()
            .filter(|x| x.game_key().keystate == choice.to_string())
            .map(|x| x.clone())
            .collect::<Vec<GameKeyModel>>(),
        None => game_keys,
    };

    if !game_keys.is_empty() {
        paginate::paginate_game_keys(ctx, &game_keys).await?;
    } else {
        ctx.reply("No games found.").await?;
    }

    Ok(())
}

/// Displays the details of a gamekey except its key.
#[poise::command(slash_command)]
pub async fn details(
    ctx: Context<'_>,
    #[description = "Id of the gamekey."] gamekey_id: i32,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let game_key = match GameKeyQuery::get_one(db, gamekey_id, ctx.author().id.get()).await? {
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
#[poise::command(slash_command, dm_only)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: String,
    #[description = "Platform of the game key."] platform: PlatformCoice,
    #[description = "State of the key."] keystate: KeystateCoice,
    #[description = "Value of the key."] value: String,
    #[description = "Store page link of the key."] page_link: Option<String>,
    #[description = "Notes for the key."] notes: Option<String>,
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
        notes: notes,
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
#[poise::command(slash_command)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Id of the gamekey to delete"] gamekey_id: i32,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let deleted_keys = GameKeyMutation::delete(db, gamekey_id, ctx.author().id.get()).await?;

    ctx.reply(format!("Deleted `{}` keys.", deleted_keys.rows_affected))
        .await?;

    warn!("Deleted game with id '{gamekey_id}'.");

    Ok(())
}

/// Edits the details of a game key.
#[poise::command(slash_command)]
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
    #[description = "Notes for the key."] notes: Option<String>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(page_link) = &page_link {
        if let Err(why) = url::Url::parse(&page_link) {
            error!("Invalid url: {}", why);

            ctx.send(
                CreateReply::default()
                    .content("The url you provided is invalid.")
                    .ephemeral(true),
            )
            .await?;

            return Ok(());
        }
    }

    if let Some(game_key) = GameKeyQuery::get_one(db, id, ctx.author().id.get()).await? {
        let game_id = if let Some(game) = game {
            match GameQuery::get_by_title(db, &game).await? {
                Some(g) => g.id,
                None => {
                    ctx.send(
                        CreateReply::default()
                            .content("Could not find game.")
                            .ephemeral(true),
                    )
                    .await?;
                    return Ok(());
                }
            }
        } else {
            game_key.game_id
        };

        let platform_id = if let Some(platform) = platform {
            match PlatformQuery::get_by_name(db, &platform.to_string()).await? {
                Some(g) => g.id,
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
            }
        } else {
            game_key.platform_id
        };

        let model = game_key::Model {
            id,
            game_id: game_id,
            platform_id: platform_id,
            value: value.unwrap_or(game_key.value),
            keystate: keystate.map(|x| x.to_string()).unwrap_or(game_key.keystate),
            page_link: page_link.or(game_key.page_link),
            notes: notes.or(game_key.notes),
            create_date: game_key.create_date,
            create_user_id: game_key.create_user_id,
            modify_date: Some(Utc::now().naive_utc().to_string()),
            modify_user_id: Some(ctx.author().id.into()),
        };

        let message = match GameKeyMutation::update(db, model).await {
            Ok(_) => "Successfully updated gamekey.",
            Err(why) => {
                error!("Could not update gamekey because of '{}'.", why);
                "Could not update the gamekey because of an internal error."
            }
        };

        ctx.send(CreateReply::default().content(message).ephemeral(true))
            .await?;
    } else {
        ctx.send(
            CreateReply::default()
                .content(format!("Could not find a game with the id '{}'.", id))
                .ephemeral(true),
        )
        .await?;
    }

    Ok(())
}

/// Claims a key. Sends the key value hidden behind a spoiler into the channel.
#[poise::command(slash_command, owners_only)]
pub async fn claim(
    ctx: Context<'_>,
    #[description = "Id of the key you want to claim."] gamekey_id: i32,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let mut game_key = match GameKeyQuery::get_one(db, gamekey_id, ctx.author().id.get()).await? {
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

/// Claims a key from a game. Sends the key value hidden behind a spoiler into the channel.
#[poise::command(slash_command, owners_only)]
pub async fn quickclaim(
    ctx: Context<'_>,
    #[description = "Name of the game you want to claim a key from."]
    #[autocomplete = "autocomplete_game"]
    game: String,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let game_id = match GameQuery::get_by_title(db, &game).await? {
        Some(g) => g.id,
        None => {
            ctx.send(
                CreateReply::default()
                    .content("Could not find game.")
                    .ephemeral(true),
            )
            .await?;
            return Ok(());
        }
    };

    if let Some(mut game_key) = GameKeyQuery::get_all_by_game(db, game_id, ctx.author().id.get())
        .await?
        .iter()
        .filter(|x| x.game_key().keystate == "Unused")
        .map(|x| x.game_key().clone())
        .next()
    {
        let reply = CreateReply::default()
            .content(format!("Your key: `{}`", game_key.value))
            .ephemeral(true);

        game_key.keystate = "Used".to_owned();
        game_key.modify_date = Some(Utc::now().naive_utc().to_string());
        game_key.modify_user_id = Some(ctx.author().id.into());

        GameKeyMutation::update(db, game_key).await?;

        ctx.send(reply).await?;
        return Ok(());
    }

    ctx.send(
        CreateReply::default()
            .content(format!("No unused keys for this game found."))
            .ephemeral(true),
    )
    .await?;

    Ok(())
}

/// Claims a key. Sends the key value hidden behind a spoiler into the channel.
#[poise::command(
    slash_command,
    owners_only,
    name_localized("de", "claim-random"),
    name_localized("en-US", "claim-random")
)]
pub async fn claim_random(ctx: Context<'_>) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let gamekeys = GameKeyQuery::get_all_ids(db, ctx.author().id.get()).await?;

    if gamekeys.is_empty() {
        ctx.send(
            CreateReply::default()
                .content("No more gamekeys are available.")
                .ephemeral(true),
        )
        .await?;

        return Ok(());
    }

    let random_number = {
        let mut rng = rand::thread_rng();
        rng.gen_range(0..gamekeys.len())
    };
    let gamekey_id = gamekeys[random_number];

    let mut game_key = match GameKeyQuery::get_one(db, gamekey_id, ctx.author().id.get()).await? {
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

    let game = GameQuery::get_one(db, game_key.game_id)
        .await?
        .expect(&format!(
            "Game with id {} has been deleted.",
            game_key.game_id
        ));

    let reply = CreateReply::default()
        .content(format!(
            "Your key: `{}` for `{}`",
            game_key.value, game.title
        ))
        .ephemeral(true);

    game_key.keystate = "Used".to_owned();
    game_key.modify_date = Some(Utc::now().naive_utc().to_string());
    game_key.modify_user_id = Some(ctx.author().id.into());

    GameKeyMutation::update(db, game_key).await?;

    ctx.send(reply).await?;
    Ok(())
}
