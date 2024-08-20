use ::entity::{
    game::{self, Entity as Game},
    game_key::{self, Entity as GameKey},
    keylist::{self, Entity as Keylist},
    keylist_access::{self, Entity as KeylistAccess},
    platform::{self, Entity as Platform},
};
use sea_orm::{
    ColumnTrait, DbConn, DbErr, EntityTrait, FromQueryResult, ModelTrait, PaginatorTrait, QueryFilter, QuerySelect,
};

#[derive(Clone, Debug, FromQueryResult)]
pub struct GameDetailModel {
    pub id: i32,
    pub title: String,
    pub description: Option<String>,
    pub create_date: String,
    pub create_user_id: i64,
    pub modify_date: Option<String>,
    pub modify_user_id: Option<i64>,
    pub image_link: Option<String>,
    pub key_count: i64,
}

pub struct GameQuery;

impl GameQuery {
    /// Gets all games from the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_all(db: &DbConn) -> Result<Vec<game::Model>, DbErr> {
        Game::find().all(db).await
    }

    /// Gets a game from the database by its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<game::Model>, DbErr> {
        Game::find_by_id(id).one(db).await
    }

    /// Gets a game by its title from the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_by_title(db: &DbConn, title: &str) -> Result<Option<game::Model>, DbErr> {
        Game::find()
            .filter(game::Column::Title.eq(title))
            .one(db)
            .await
    }

    /// Checks whether a game exists by its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn exists(db: &DbConn, id: i32) -> Result<bool, DbErr> {
        let game = Game::find_by_id(id).one(db).await?;

        Ok(game.is_some())
    }

    pub async fn count_total(db: &DbConn) -> Result<u64, DbErr> {
        Game::find().count(db).await
    }

    pub async fn get_gamedetails(db: &DbConn) -> Result<Vec<GameDetailModel>, DbErr> {
        Game::find()
            .column_as(game_key::Column::Id.count(), "key_count")
            .left_join(GameKey)
            .into_model::<GameDetailModel>()
            .all(db).await
    }
}

/// Model for querying all data about a gamekey.
#[derive(Clone)]
pub struct GameKeyModel {
    game_key: game_key::Model,
    game: game::Model,
    platform: platform::Model,
}

impl GameKeyModel {
    #[must_use]
    pub fn game_key(&self) -> &game_key::Model {
        &self.game_key
    }

    #[must_use]
    pub fn game(&self) -> &game::Model {
        &self.game
    }

    #[must_use]
    pub fn platform(&self) -> &platform::Model {
        &self.platform
    }
}

pub struct GameKeyQuery;

impl GameKeyQuery {
    /// Gets all gamekeys in the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_all(db: &DbConn) -> Result<Vec<game_key::Model>, DbErr> {
        GameKey::find().all(db).await
    }

    /// Gets a gamekey by its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_one(
        db: &DbConn,
        id: i32,
        user_id: u64,
    ) -> Result<Option<game_key::Model>, DbErr> {
        GameKey::find_by_id(id)
            .filter(game_key::Column::CreateUserId.eq(user_id))
            .one(db)
            .await
    }

    /// Gets all gamekeys filtered by game.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_all_by_game(
        db: &DbConn,
        game_id: i32,
        user_id: u64,
    ) -> Result<Vec<GameKeyModel>, DbErr> {
        let game_keys = GameKey::find()
            .filter(
                game_key::Column::GameId
                    .eq(game_id)
                    .and(game_key::Column::CreateUserId.eq(user_id)),
            )
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
                game,
                platform,
            });
        }

        Ok(complete_models)
    }

    /// Gets all gamekeys filtered by platform.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
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

    /// Gets all gamekeys filtered by platform and game.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
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

    /// Gets the number of gamekeys found for a game id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn count_by_game(db: &DbConn, game_id: i32) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::GameId.eq(game_id))
            .count(db)
            .await
    }

    /// Gets all unused gamekey ids
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_all_ids(db: &DbConn, user_id: u64) -> Result<Vec<i32>, DbErr> {
        let res: Vec<i32> = GameKey::find()
            .select_only()
            .column(game_key::Column::Id)
            .filter(
                game_key::Column::Keystate
                    .eq("Unused")
                    .and(game_key::Column::CreateUserId.eq(user_id)),
            )
            .into_tuple()
            .all(db)
            .await?;

        Ok(res)
    }

    pub async fn count_total(db: &DbConn) -> Result<u64, DbErr> {
        GameKey::find().count(db).await
    }

    pub async fn count_total_of_user(db: &DbConn, user_id: u64) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::CreateUserId.eq(user_id))
            .count(db)
            .await
    }

    pub async fn count_unused(db: &DbConn) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::Keystate.eq("Unused"))
            .count(db)
            .await
    }

    pub async fn count_unused_of_user(db: &DbConn, user_id: u64) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(
                game_key::Column::Keystate
                    .eq("Unused")
                    .and(game_key::Column::CreateUserId.eq(user_id)),
            )
            .count(db)
            .await
    }

    pub async fn count_used(db: &DbConn) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(game_key::Column::Keystate.eq("Used"))
            .count(db)
            .await
    }

    pub async fn count_used_of_user(db: &DbConn, user_id: u64) -> Result<u64, DbErr> {
        GameKey::find()
            .filter(
                game_key::Column::Keystate
                    .eq("Used")
                    .and(game_key::Column::CreateUserId.eq(user_id)),
            )
            .count(db)
            .await
    }
}

pub struct PlatformQuery;

impl PlatformQuery {
    /// Gets all platforms in the database.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_all(db: &DbConn) -> Result<Vec<platform::Model>, DbErr> {
        Platform::find().all(db).await
    }

    /// Gets a platform based on its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_one(db: &DbConn, id: i32) -> Result<Option<platform::Model>, DbErr> {
        Platform::find_by_id(id).one(db).await
    }

    /// Gets a platform based on its name.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn get_by_name(db: &DbConn, name: &str) -> Result<Option<platform::Model>, DbErr> {
        Platform::find()
            .filter(platform::Column::Name.eq(name))
            .one(db)
            .await
    }
}

pub struct KeylistQuery;

impl KeylistQuery {
    /// Gets all owned keylists.
    pub async fn get_owned_keylists(
        db: &DbConn,
        user_id: u64,
    ) -> Result<Vec<keylist::Model>, DbErr> {
        Keylist::find()
            .filter(keylist::Column::OwnerId.eq(user_id))
            .all(db)
            .await
    }

    /// Gets all assigned keylists.
    pub async fn get_assigned_keylists(
        db: &DbConn,
        user_id: u64,
    ) -> Result<Vec<keylist::Model>, DbErr> {
        Keylist::find()
            .left_join(KeylistAccess)
            .filter(keylist_access::Column::TargetUserId.eq(user_id))
            .all(db)
            .await
    }

    /// Gets all assigned and owned keylists.
    pub async fn get_keylists(db: &DbConn, user_id: u64) -> Result<Vec<keylist::Model>, DbErr> {
        Keylist::find()
            .left_join(KeylistAccess)
            .filter(
                keylist_access::Column::TargetUserId
                    .eq(user_id)
                    .or(keylist::Column::OwnerId.eq(user_id)),
            )
            .all(db)
            .await
    }
}
/*
SELECT *
FROM keylist AS k
LEFT JOIN keylist_access AS ka
ON k.id = ka.keylist_id
WHERE ka.target_user_id = 0;
*/
