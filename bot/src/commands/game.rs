use std::default;

use chrono::Utc;
use entity::game;
use gemuki_service::{mutation::GameMutation, query::GameQuery};
use log::error;
use migration::DbErr;
use poise::serenity_prelude::{self as serenity, CreateMessage};

use crate::{paginate, Data};

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
pub async fn list(ctx: Context<'_>) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let games = GameQuery::get_all(db).await?;

    if !games.is_empty() {
        paginate::paginate_games(ctx, &games).await?;
    }
    else {
        ctx.reply("No games found.").await?;
    }

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
    ctx: Context<'_>,
    #[description = "Title of the game you want to add."] title: String,
    #[description = "Description of the game you want to add. Optional."] description: Option<
        String,
    >,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let model = game::Model {
        id: 0,
        title,
        description,
        create_date: Utc::now().naive_utc().to_string(),
        create_user_id: ctx.author().id.into(),
        modify_date: None,
        modify_user_id: None,
    };

    let message = match GameMutation::create(db, model).await {
        Ok(_) => "Successfully added game.",
        Err(why) if why == DbErr::RecordNotInserted => {
            error!("Could not insert game because of '{}'", why);
            
            "Could not add game because a game with that title already exists."
        },
        Err(why) => {
            error!("Could not insert game because of '{}'", why);

            "Could not add game because of an internal error."
        },
    };

    ctx.reply(message).await?;

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
