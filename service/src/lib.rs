use entity::{game_key, prelude::GameKey};
use sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, QuerySelect};

pub mod mutation;
pub mod query;

/// Counts all users that created gamekeys in the database.
///
/// # Errors
///
/// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
pub async fn count_users(db: &DbConn) -> Result<u64, DbErr> {
    GameKey::find()
        .select_only()
        .column(game_key::Column::CreateUserId)
        .distinct()
        .count(db)
        .await
}
