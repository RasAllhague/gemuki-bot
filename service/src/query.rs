use ::entity::{
    game::{self, Entity as Game},
    game_key::{self, Entity as GameKey},
    platform::{self, Entity as Platform},
};
use sea_orm::*;

pub struct GameQuery;

pub struct GameKeyQuery;

pub struct PlatformQuery;

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

#[derive(Clone)]
pub struct GameKeyModel {
    game_key: game_key::Model,
    game: game::Model,
    platform: platform::Model,
}

impl GameKeyModel {
    pub fn game_key(&self) -> &game_key::Model {
        &self.game_key
    }
    
    pub fn game(&self) -> &game::Model {
        &self.game
    }
    
    pub fn platform(&self) -> &platform::Model {
        &self.platform
    }
}

impl GameKeyQuery {
    pub async fn get_all(db: &DbConn) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find().all(db).await
    }

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<game_key::Model>, DbErr> {
        GameKey::find_by_id(id).one(db).await
    }

    pub async fn get_all_by_game(db: &DbConn, game_id: i32) -> Result<Vec<GameKeyModel>, DbErr> {
        let game_keys = GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .all(db)
            .await?;

        let mut complete_models: Vec<GameKeyModel> = Vec::new();

        for game_key in game_keys {
            let platform = game_key
                .find_related(Platform)
                .one(db)
                .await?
                .ok_or(DbErr::Custom(
                    "No platform bound to this gamekey".to_owned(),
                ))?;
            let game = game_key
                .find_related(Game)
                .one(db)
                .await?
                .ok_or(DbErr::Custom("No game bound to this gamekey".to_owned()))?;

            complete_models.push(GameKeyModel {
                game_key,
                platform,
                game,
            });
        }

        Ok(complete_models)
    }

    pub async fn get_all_by_platform(
        db: &DbConn,
        platform_id: i32,
    ) -> Result<Vec<(game_key::Model, Option<platform::Model>)>, DbErr> {
        GameKey::find()
            .filter(game_key::Column::PlatformId.eq(platform_id))
            .find_also_related(Platform)
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

    pub async fn count_by_game(db: &DbConn, game_id: i32) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .count(db)
            .await
    }
}

impl PlatformQuery {
    pub async fn get_all(db: &DbConn) -> Result<Vec<platform::Model>, DbErr> {
        Platform::find().all(db).await
    }

    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<platform::Model>, DbErr> {
        Platform::find_by_id(id).one(db).await
    }

    pub async fn get_by_name(db: &DbConn, name: &str) -> Result<Option<platform::Model>, DbErr> {
        Platform::find()
            .filter(platform::Column::Name.eq(name))
            .one(db)
            .await
    }
}
