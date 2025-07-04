use ::entity::{
    game::{self, Entity as Game},
    game_key::{self, Entity as GameKey},
};

use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, DeleteResult, EntityTrait, QueryFilter, Set,
};

pub struct GameMutation;

impl GameMutation {
    /// Creates a new game.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn create(db: &DbConn, game: game::Model) -> Result<game::Model, DbErr> {
        game::ActiveModel {
            title: Set(game.title),
            description: Set(game.description),
            image_link: Set(game.image_link),
            create_date: Set(game.create_date.clone()),
            create_user_id: Set(game.create_user_id),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates the details of a game.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn update(
        db: &DbConn,
        update_game: game::Model,
    ) -> Result<Option<game::Model>, DbErr> {
        let game: game::ActiveModel = match Game::find_by_id(update_game.id).one(db).await? {
            Some(m) => m.into(),
            None => return Ok(None),
        };

        let updated = game::ActiveModel {
            id: game.id,
            title: Set(update_game.title),
            description: Set(update_game.description),
            image_link: Set(update_game.image_link),
            create_date: game.create_date,
            create_user_id: game.create_user_id,
            modify_date: Set(update_game.modify_date),
            modify_user_id: Set(update_game.modify_user_id),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }

    /// Deletes a gamekey by its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn delete(db: &DbConn, id: i32) -> Result<DeleteResult, DbErr> {
        Game::delete_by_id(id).exec(db).await
    }
}

pub struct GameKeyMutation;

impl GameKeyMutation {
    /// Creates a new gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn create(db: &DbConn, gamekey: game_key::Model) -> Result<game_key::Model, DbErr> {
        game_key::ActiveModel {
            game_id: Set(gamekey.game_id),
            platform_id: Set(gamekey.platform_id),
            value: Set(gamekey.value),
            keystate: Set(gamekey.keystate),
            page_link: Set(gamekey.page_link),
            create_date: Set(gamekey.create_date),
            create_user_id: Set(gamekey.create_user_id),
            expiration_date: Set(gamekey.expiration_date),
            ..Default::default()
        }
        .insert(db)
        .await
    }

    /// Updates the details of a gamekey.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn update(
        db: &DbConn,
        update_gamekey: game_key::Model,
    ) -> Result<Option<game_key::Model>, DbErr> {
        let gamekey: game_key::ActiveModel =
            match GameKey::find_by_id(update_gamekey.id).one(db).await? {
                Some(m) => m.into(),
                None => return Ok(None),
            };

        let updated = game_key::ActiveModel {
            id: gamekey.id,
            game_id: Set(update_gamekey.game_id),
            platform_id: Set(update_gamekey.platform_id),
            value: Set(update_gamekey.value),
            keystate: Set(update_gamekey.keystate),
            page_link: Set(update_gamekey.page_link),
            notes: Set(update_gamekey.notes),
            create_date: gamekey.create_date,
            create_user_id: gamekey.create_user_id,
            modify_date: Set(update_gamekey.modify_date),
            modify_user_id: Set(update_gamekey.modify_user_id),
            expiration_date: Set(update_gamekey.expiration_date),
        }
        .update(db)
        .await?;

        Ok(Some(updated))
    }

    /// Deletes a gamekey by its id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn delete(db: &DbConn, id: i32, user_id: u64) -> Result<DeleteResult, DbErr> {
        GameKey::delete_by_id(id)
            .filter(game_key::Column::CreateUserId.eq(user_id))
            .exec(db)
            .await
    }

    /// Deletes all game keys by game id.
    ///
    /// # Errors
    ///
    /// Will return `Err` if database operation fail. For more information look at [DbErr](https://docs.rs/sea-orm/latest/sea_orm/error/enum.DbErr.html).
    pub async fn delete_by_game(db: &DbConn, game_id: i32) -> Result<DeleteResult, DbErr> {
        GameKey::delete_many()
            .filter(game_key::Column::GameId.eq(game_id))
            .exec(db)
            .await
    }
}
