pub mod game;
pub mod gamekey;
pub mod version;
pub mod statistic;

use crate::{Data, PoiseError};
use poise::serenity_prelude::futures::{self, Stream, StreamExt};

pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

async fn autocomplete_game<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let mut cache = ctx.data().cache.lock().await;
    let db = &ctx.data().conn;

    cache.update(db).await;

    let title = cache.cache().to_vec();

    futures::stream::iter(title)
        .filter(move |name| {
            futures::future::ready(name.to_lowercase().starts_with(&partial.to_lowercase()))
        })
        .map(|name| name.to_string())
}
