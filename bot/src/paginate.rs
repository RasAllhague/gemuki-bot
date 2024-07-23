//! Sample pagination implementation

use crate::Data;
use entity::game;
use gemuki_service::query::GameKeyQuery;
use poise::serenity_prelude::{self as serenity, Color, CreateEmbed};

type PoiseError = Box<dyn std::error::Error + Send + Sync>;
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
