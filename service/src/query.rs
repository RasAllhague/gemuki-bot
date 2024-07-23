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

    pub async fn get_by_title(db: &DbConn, title: &str) -> Result<Option<game::Model>, DbErr> {
        Game::find()
            .filter(game::Column::Title.eq(title))
            .one(db)
            .await
    }

    pub async fn exists(db: &DbConn, id: i32) -> Result<bool, DbErr> {
        let game = Game::find_by_id(id).one(db).await?;

        Ok(game.is_some())
    }
}

impl GameKeyQuery {
    pub async fn get_all(
        db: &DbConn,
    ) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find()
            .all(db)
            .await
    }

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<game_key::Model>, DbErr> {
        GameKey::find_by_id(id).one(db).await
    }

    pub async fn get_all_by_game(
        db: &DbConn,
        game_id: i32,
    ) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .all(db)
            .await
    }

    pub async fn get_all_by_platform(
        db: &DbConn,
        platform_id: i32,
    ) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find()
            .filter(game_key::Column::PlatformId.eq(platform_id))
            .all(db)
            .await
    }

    pub async fn get_all_filtered(
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

    pub async fn count_by_game(
        db: &DbConn,
        game_id: i32,
    ) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .count(db)
            .await
    }
}
