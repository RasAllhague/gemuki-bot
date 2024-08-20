use entity::{game_key, prelude::GameKey};
use sea_orm::{DbConn, DbErr, EntityTrait, PaginatorTrait, QuerySelect};

pub mod mutation;
pub mod query;

pub async fn count_users(db: &DbConn) -> Result<u64, DbErr> {
    GameKey::find()
        .select_only()
        .column(game_key::Column::CreateUserId)
        .distinct()
        .count(db)
        .await
}
