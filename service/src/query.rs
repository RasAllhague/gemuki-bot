use ::entity::{
    game::{self, Entity as Game},
    game_key::{self, Entity as GameKey},
};
use sea_orm::*;

pub struct GameQuery;

pub struct GameKeyQuery;

impl GameQuery {
    pub async fn get_all(db: &DbConn) -> Result<Vec<game::Model>, DbErr> {
        Game::find().all(db).await
    }

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<game::Model>, DbErr> {
        Game::find_by_id(id).one(db).await
    }
}

impl GameKeyQuery {
    pub async fn get_all(
        db: &DbConn,
        game_id: i32,
        platform_id: i32,
    ) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .filter(game_key::Column::PlatformId.eq(platform_id))
            .all(db)
            .await
    }

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<game_key::Model>, DbErr> {
        GameKey::find_by_id(id).one(db).await
    }
}
