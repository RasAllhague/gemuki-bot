pub mod game;
pub mod gamekey;
pub mod keylist;
pub mod statistic;
pub mod version;

use crate::{Data, PoiseError};
use gemuki_service::query::KeylistQuery;
use log::error;
use poise::serenity_prelude::futures::{self, Stream, StreamExt};

pub type Context<'a> = poise::Context<'a, Data, PoiseError>;

async fn autocomplete_game<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let mut cache = ctx.data().game_title_cache.lock().await;
    let db = &ctx.data().conn;

    cache.update(db).await;

    let title = cache.cache().to_vec();

    futures::stream::iter(title)
        .filter(move |name| {
            futures::future::ready(name.to_lowercase().starts_with(&partial.to_lowercase()))
        })
        .map(|name| name.to_string())
}

async fn autocomplete_keylist<'a>(
    ctx: Context<'_>,
    partial: &'a str,
) -> impl Stream<Item = String> + 'a {
    let db = &ctx.data().conn;

    let keylists = match KeylistQuery::get_owned_keylists(db, ctx.author().id.into()).await {
        Ok(k) => k.iter().map(|x| x.clone().name).collect::<Vec<String>>(),
        Err(why) => {
            error!("Could not get keylists from the database, {why}");
            Vec::new()
        }
    };

    futures::stream::iter(keylists)
        .filter(move |name| {
            futures::future::ready(name.to_lowercase().starts_with(&partial.to_lowercase()))
        })
        .map(|name| name.to_string())
}
