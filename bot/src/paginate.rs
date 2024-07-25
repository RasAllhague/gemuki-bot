use crate::{Data, PoiseError};
use entity::game;
use gemuki_service::query::{GameKeyModel, GameKeyQuery};
use poise::serenity_prelude::{self as serenity, Color, CreateEmbed};

type Context<'a> = poise::Context<'a, Data, PoiseError>;

pub async fn paginate_games(ctx: Context<'_>, pages: &[game::Model]) -> Result<(), PoiseError> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let db = &ctx.data().conn;

    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        let embed = create_gamedetail_embed(pages, db, 0).await?;

        poise::CreateReply::default()
            .embed(embed)
            .components(vec![components])
    };

    ctx.send(reply).await?;

    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }

        let embed = create_gamedetail_embed(pages, db, current_page).await?;

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(embed),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn create_gamedetail_embed(
    pages: &[game::Model],
    db: &migration::sea_orm::DatabaseConnection,
    current_page: usize,
) -> Result<CreateEmbed, PoiseError> {
    let game = pages[current_page].clone();
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

    Ok(embed)
}

pub async fn paginate_game_keys(
    ctx: Context<'_>,
    pages: &[GameKeyModel],
) -> Result<(), PoiseError> {
    let ctx_id = ctx.id();
    let prev_button_id = format!("{}prev", ctx_id);
    let next_button_id = format!("{}next", ctx_id);

    let reply = {
        let components = serenity::CreateActionRow::Buttons(vec![
            serenity::CreateButton::new(&prev_button_id).emoji('◀'),
            serenity::CreateButton::new(&next_button_id).emoji('▶'),
        ]);

        let embed = create_gamekey_detail_embed(pages, 0).await?;

        poise::CreateReply::default()
            .embed(embed)
            .components(vec![components])
    };

    ctx.send(reply).await?;

    let mut current_page = 0;
    while let Some(press) = serenity::collector::ComponentInteractionCollector::new(ctx)
        .filter(move |press| press.data.custom_id.starts_with(&ctx_id.to_string()))
        .timeout(std::time::Duration::from_secs(3600 * 24))
        .await
    {
        if press.data.custom_id == next_button_id {
            current_page += 1;
            if current_page >= pages.len() {
                current_page = 0;
            }
        } else if press.data.custom_id == prev_button_id {
            current_page = current_page.checked_sub(1).unwrap_or(pages.len() - 1);
        } else {
            continue;
        }

        let embed = create_gamekey_detail_embed(pages, current_page).await?;

        press
            .create_response(
                ctx.serenity_context(),
                serenity::CreateInteractionResponse::UpdateMessage(
                    serenity::CreateInteractionResponseMessage::new().embed(embed),
                ),
            )
            .await?;
    }

    Ok(())
}

async fn create_gamekey_detail_embed(
    pages: &[GameKeyModel],
    current_page: usize,
) -> Result<CreateEmbed, PoiseError> {
    let model = pages[current_page].clone();
    let game_key = model.game_key().clone();
    let game = model.game().clone();
    let platform = model.platform().clone();

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

    Ok(embed)
}
