use chrono::Utc;
use entity::{keylist, keylist_key};
use gemuki_service::{
    mutation::{KeylistKeyMutation, KeylistMutation},
    query::KeylistQuery,
};
use log::error;
use migration::{sea_orm::DbConn, DbErr};
use poise::serenity_prelude::CreateMessage;

use crate::{commands::autocomplete_keylist, paginate, Data, PoiseError};

type Context<'a> = poise::Context<'a, Data, PoiseError>;

#[derive(Clone, Copy, Default, Debug, poise::ChoiceParameter)]
pub enum KeylistOrigin {
    #[default]
    All,
    Owned,
    Assigned,
}

/// A command for managing games.
#[poise::command(slash_command, subcommands("list", "create", "add"))]
pub async fn keylist(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

/// Lists all keylists a user has access to in a paginated result.
#[poise::command(slash_command)]
pub async fn list(
    ctx: Context<'_>,
    #[description = "Filter for the origin of the key."] origin: KeylistOrigin,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let keylists = get_keylists(db, ctx.author().id.get(), origin).await?;

    if keylists.is_empty() {
        ctx.say("No keylists have been found for you.").await?;

        return Ok(());
    }

    paginate::create_pagination(ctx, &keylists).await?;

    Ok(())
}

/// Creates a new keylist.
#[poise::command(slash_command)]
pub async fn create(
    ctx: Context<'_>,
    #[description = "Name of the keylist. Must be unique to your user."] name: String,
    #[description = "Description of the keylist. Optional."] description: Option<String>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;

    let model = keylist::Model {
        id: 0,
        name,
        description,
        owner_id: ctx.author().id.into(),
        create_date: Utc::now().naive_utc().to_string(),
        create_user_id: ctx.author().id.into(),
        modify_date: None,
        modify_user_id: None,
    };

    let message = match KeylistMutation::create(db, model).await {
        Ok(_) => "Successfully create keylist.",
        Err(why) => {
            error!("Could not insert new keylist because of '{}'.", why);
            "Could not create keylist because of an internal server error."
        }
    };

    ctx.reply(message).await?;

    Ok(())
}

/// Adds keys to a keylist.
#[poise::command(slash_command)]
pub async fn add(
    ctx: Context<'_>,
    #[description = "Name of the keylist."]
    #[autocomplete = "autocomplete_keylist"]
    name: String,
    #[description = "Id of the first key to add."] key_id: i32,
    #[description = "Id of an key to add."] key_id2: Option<i32>,
    #[description = "Id of an key to add."] key_id3: Option<i32>,
    #[description = "Id of an key to add."] key_id4: Option<i32>,
    #[description = "Id of an key to add."] key_id5: Option<i32>,
    #[description = "Id of an key to add."] key_id6: Option<i32>,
    #[description = "Id of an key to add."] key_id7: Option<i32>,
    #[description = "Id of an key to add."] key_id8: Option<i32>,
) -> Result<(), PoiseError> {
    let db = &ctx.data().conn;
    let author_id: i64 = ctx.author().id.into();

    let selected_keylist = match KeylistQuery::get_by_name(db, &name, author_id).await? {
        Some(k) => k,
        None => {
            ctx.reply("Could not find keylist.").await?;
            return Ok(());
        }
    };

    let message = match create_keylist_key(db, selected_keylist.id, key_id, author_id).await {
        Ok(_) => format!(
            "Successfully added key `{}` to keylist `{}`",
            key_id, selected_keylist.name
        ),
        Err(why) => {
            error!(
                "Failed to add key '{}' to keylist '{}', {why}",
                key_id, selected_keylist.name
            );
            format!(
                "Successfully added key `{}` to keylist `{}`",
                key_id, selected_keylist.name
            )
        }
    };

    ctx.reply(message).await?;

    if let Some(key_id) = key_id2 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id3 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id4 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id5 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id6 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id7 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }
    if let Some(key_id) = key_id8 {
        create_and_send_keylist_key(ctx, key_id, db, &selected_keylist, author_id).await?;
    }

    Ok(())
}

async fn get_keylists(
    db: &DbConn,
    user_id: u64,
    origin: KeylistOrigin,
) -> Result<Vec<keylist::Model>, DbErr> {
    match origin {
        KeylistOrigin::All => KeylistQuery::get_keylists(db, user_id).await,
        KeylistOrigin::Owned => KeylistQuery::get_owned_keylists(db, user_id).await,
        KeylistOrigin::Assigned => KeylistQuery::get_assigned_keylists(db, user_id).await,
    }
}

async fn create_and_send_keylist_key(
    ctx: Context<'_>,
    key_id: i32,
    db: &DbConn,
    keylist: &keylist::Model,
    author_id: i64,
) -> Result<(), PoiseError> {
    let message = match create_keylist_key(db, keylist.id, key_id, author_id).await {
        Ok(_) => format!(
            "Successfully added key `{}` to keylist `{}`",
            key_id, keylist.name
        ),
        Err(why) => {
            error!(
                "Failed to add key '{}' to keylist '{}', {why}",
                key_id, keylist.name
            );
            format!(
                "Successfully added key `{}` to keylist `{}`",
                key_id, keylist.name
            )
        }
    };

    ctx.channel_id()
        .send_message(ctx.http(), CreateMessage::new().content(message))
        .await?;

    Ok(())
}

async fn create_keylist_key(
    db: &DbConn,
    keylist_id: i32,
    gamekey_id: i32,
    create_user_id: i64,
) -> Result<keylist_key::Model, DbErr> {
    let model = keylist_key::Model {
        id: 0,
        keylist_id,
        gamekey_id,
        create_date: Utc::now().naive_utc().to_string(),
        create_user_id,
    };

    KeylistKeyMutation::create(db, model).await
}
