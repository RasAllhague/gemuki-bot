use entity::keylist;
use gemuki_service::query::KeylistQuery;
use migration::{sea_orm::DbConn, DbErr};

use crate::{paginate, Data, PoiseError};

type Context<'a> = poise::Context<'a, Data, PoiseError>;

#[derive(Clone, Copy, Default, Debug, poise::ChoiceParameter)]
pub enum KeylistOrigin {
    #[default]
    All,
    Owned,
    Assigned,
}

/// A command for managing games.
#[poise::command(slash_command, subcommands("list", "create"))]
pub async fn keylist(ctx: Context<'_>) -> Result<(), PoiseError> {
    ctx.say("How did you manage to do this?").await?;
    Ok(())
}

pub async fn get_keylists(
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
    ctx.say("Not yet implemented").await?;
    Ok(())
}
