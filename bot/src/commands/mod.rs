pub mod game;
pub mod gamekey;
pub mod version;

use gemuki_service::query::GameQuery;
use poise::serenity_prelude::futures::{self, Stream, StreamExt};
use crate::{Data, PoiseError};

pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

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