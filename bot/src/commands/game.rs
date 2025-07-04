use std::path::Path;

use crate::{commands::autocomplete_game, paginate, steam, Data, PoiseError};
use chrono::Utc;
use entity::game;
use gemuki_service::{
    mutation::{GameKeyMutation, GameMutation},
    query::{GameKeyQuery, GameQuery},
};
use log::{error, info, warn};
use migration::sea_orm::DbConn;
use poise::{
    serenity_prelude::{Color, CreateAttachment, CreateEmbed},
    CreateReply,
};
use tempfile::tempfile;
use tokio::{fs, io::AsyncWriteExt};

type Context<'a> = poise::Context<'a, Data, PoiseError>;

/// A command for managing games.
#[poise::command(
    slash_command,
    owners_only,
    subcommands("list", "details", "add", "edit", "remove", "quicksetup", "export")
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
        create_date: Utc::now(),
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

    ctx.data()
        .game_title_cache
        .lock()
        .await
        .force_update(db)
        .await;

    ctx.reply(message).await?;

    Ok(())
}

/// Creates a new game based on steamshop info.
#[poise::command(slash_command, owners_only)]
pub async fn quicksetup(
    ctx: Context<'_>,
    #[description = "Title of the game you want to add."] title: String,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    if let Some(_) = GameQuery::get_by_title(db, &title).await? {
        ctx.reply("Could not add game because it already exists.")
            .await?;
        return Ok(());
    };

    let mut cache = ctx.data().steam_app_cache.lock().await;
    cache.update().await;

    let app = match cache.find_by_name(&title) {
        Some(app) => app,
        None => {
            ctx.reply("The title you search for does not exist on steam.")
                .await?;
            return Ok(());
        }
    };

    let app_details = match steam::get_app_details(app.appid()).await {
        Ok(None) => {
            ctx.reply("Could not retrieve game data from steam.")
                .await?;
            return Ok(());
        }
        Ok(details) => details.unwrap(),
        Err(why) => {
            error!("Could not retrieve game data from steam, {:?}", why);
            ctx.reply("Could not retrieve game data from steam.")
                .await?;
            return Ok(());
        }
    };

    let model = game::Model {
        id: 0,
        title: app_details.name.clone(),
        description: Some(app_details.short_description.clone()),
        image_link: Some(app_details.header_image.clone()),
        create_date: Utc::now(),
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

    ctx.data()
        .game_title_cache
        .lock()
        .await
        .force_update(db)
        .await;

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
            modify_date: Some(Utc::now()),
            modify_user_id: Some(ctx.author().id.into()),
        };

        let message = match GameMutation::update(db, model).await {
            Ok(_) => "Successfully updated game.",
            Err(why) => {
                error!("Could not update game because of '{}'.", why);
                "Could not update the game because of an internal error."
            }
        };

        ctx.data()
            .game_title_cache
            .lock()
            .await
            .force_update(db)
            .await;

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

/// Exports a list of all games which have unused keys.
#[poise::command(slash_command, owners_only)]
pub async fn export(ctx: Context<'_>) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let file_id: uuid::Uuid = uuid::Uuid::new_v4();
    let file_name = format!("games_{}.csv", file_id);

    write_to_file(db, ctx.author().id.get(), &file_name).await?;

    {
        let file = fs::File::open(&file_name).await?;
        let attachment = CreateAttachment::file(&file, &file_name).await?;

        ctx.send(
            CreateReply::default()
                .content(format!("Found games:"))
                .attachment(attachment),
        )
        .await?;
    }

    fs::remove_file(&file_name).await?;

    Ok(())
}

async fn write_to_file(
    db: &DbConn,
    user_id: u64,
    path: impl AsRef<Path>,
) -> Result<(), PoiseError> {
    let mut file = fs::File::create(&path).await?;

    file.write(b"game_title;create_date\n").await?;

    for game in GameQuery::get_all_games_with_keys(db, user_id).await? {
        file.write(format!("{};{}\n", game.title, game.create_date).as_bytes())
            .await?;
    }

    file.flush().await?;

    Ok(())
}
