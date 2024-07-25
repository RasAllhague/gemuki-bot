use crate::{commands::autocomplete_game, paginate, Data, PoiseError};
use chrono::Utc;
use entity::game;
use gemuki_service::{
    mutation::{GameKeyMutation, GameMutation},
    query::{GameKeyQuery, GameQuery},
};
use log::{error, warn};
use poise::{
    serenity_prelude::{Color, CreateEmbed},
    CreateReply,
};

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
    } else {
        ctx.reply("No games found.").await?;
    }

    Ok(())
}

/// Displays all details currently available to a game.
#[poise::command(slash_command, owners_only)]
pub async fn details(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: String,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(game) = GameQuery::get_by_title(db, &game).await? {
        let key_count = GameKeyQuery::count_by_game(db, game.id).await?;

        let embed = CreateEmbed::new()
            .colour(Color::DARK_BLUE)
            .title(game.title)
            .description(game.description.unwrap_or("None".to_owned()))
            .field("Id", format!("{}", game.id), true)
            .field("Keys", key_count.to_string(), true);
        let embed = match game.image_link {
            Some(link) => embed.image(link),
            None => embed,
        };

        ctx.send(CreateReply::default().embed(embed)).await?;
    } else {
        ctx.reply(format!("Could not find a game with the title '{}'.", game))
            .await?;
    }

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
    #[description = "Image link for the game. Optional."] image_link: Option<String>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(_) = GameQuery::get_by_title(db, &title).await? {
        ctx.reply("Could not add game because it already exists.")
            .await?;
        return Ok(());
    };

    if let Some(link) = &image_link {
        if let Err(why) = url::Url::parse(&link) {
            error!("Invalid url: {}", why);
            ctx.reply("The url you provided is invalid.").await?;
            return Ok(());
        }
    }

    let model = game::Model {
        id: 0,
        title,
        description,
        image_link: image_link,
        create_date: Utc::now().naive_utc().to_string(),
        create_user_id: ctx.author().id.into(),
        modify_date: None,
        modify_user_id: None,
    };

    let message = match GameMutation::create(db, model).await {
        Ok(_) => "Successfully added game.",
        Err(why) => {
            error!("Could not insert new game because of '{}'.", why);
            "Could not add game because of an internal server error."
        }
    };

    ctx.reply(message).await?;

    Ok(())
}

/// Edits details of a game.
#[poise::command(slash_command, owners_only)]
pub async fn edit(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: String,
    #[description = "Title of the game you want to edit."] title: Option<String>,
    #[description = "Description of the game you want to edit."] description: Option<String>,
    #[description = "Picture link for the image of the game."] image_link: Option<String>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(link) = &image_link {
        if let Err(why) = url::Url::parse(&link) {
            error!("Invalid url: {}", why);
            ctx.reply("The url you provided is invalid.").await?;
            return Ok(());
        }
    }

    if let Some(game) = GameQuery::get_by_title(db, &game).await? {
        let model = game::Model {
            id: game.id,
            title: title.unwrap_or(game.title),
            description: description.or(game.description),
            image_link: image_link.or(game.image_link),
            create_date: game.create_date,
            create_user_id: game.create_user_id,
            modify_date: Some(Utc::now().naive_utc().to_string()),
            modify_user_id: Some(ctx.author().id.into()),
        };

        let message = match GameMutation::update(db, model).await {
            Ok(_) => "Successfully updated game.",
            Err(why) => {
                error!("Could not update game because of '{}'.", why);
                "Could not update the game because of an internal error."
            }
        };

        ctx.reply(message).await?;
    } else {
        ctx.reply(format!("Could not find a game with title '{}'.", game))
            .await?;
    }

    Ok(())
}

/// Removes a game entry. Use on own risk as it also clears KEYs connected to the game.
#[poise::command(slash_command, owners_only)]
pub async fn remove(
    ctx: Context<'_>,
    #[description = "Name of the game."]
    #[autocomplete = "autocomplete_game"]
    game: String,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(game) = GameQuery::get_by_title(db, &game).await? {
        let deleted_keys = GameKeyMutation::delete_by_game(db, game.id).await?;
        let deleted_games = GameMutation::delete(db, game.id).await?;

        ctx.reply(format!(
            "Deleted `{}` keys and `{}` games.",
            deleted_keys.rows_affected, deleted_games.rows_affected
        ))
        .await?;

        warn!("Deleted game with title '{}'.", game.title);
    } else {
        ctx.reply(format!("No game with title {} found.", game))
            .await?;
    }

    Ok(())
}
