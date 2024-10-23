use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Game::Table)
                    .if_not_exists()
                    .col(pk_auto(Game::Id))
                    .col(string_len(Game::Title, 255))
                    .col(string_len_null(Game::Description, 500))
                    .col(timestamp(Game::CreateDate))
                    .col(big_integer(Game::CreateUserId))
                    .col(timestamp_null(Game::ModifyDate))
                    .col(big_integer_null(Game::ModifyUserId))
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GameKey::Table)
                    .if_not_exists()
                    .col(pk_auto(GameKey::Id))
                    .col(integer(GameKey::GameId))
                    .col(integer(GameKey::PlatformId))
                    .col(string_len(GameKey::Value, 255))
                    .col(enumeration(
                        GameKey::Keystate,
                        Alias::new("keystate"),
                        KeyState::iter(),
                    ))
                    .col(string_len_null(GameKey::PageLink, 500))
                    .col(timestamp(GameKey::CreateDate))
                    .col(big_integer(GameKey::CreateUserId))
                    .col(timestamp_null(GameKey::ModifyDate))
                    .col(big_integer_null(GameKey::ModifyUserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameKey::Table, GameKey::GameId)
                            .to(Game::Table, Game::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(GameKey::Table, GameKey::PlatformId)
                            .to(Platform::Table, Platform::Id),
                    )
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Platform::Table)
                    .if_not_exists()
                    .col(pk_auto(Platform::Id))
                    .col(string_len(Platform::Name, 255).unique_key())
                    .col(string_len_null(Platform::StoreLink, 500))
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(GameKey::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Platform::Table).to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(Game::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    Id,
    Title,
    Description,
    CreateDate,
    CreateUserId,
    ModifyDate,
    ModifyUserId,
}

#[derive(DeriveIden)]
enum GameKey {
    Table,
    Id,
    GameId,
    PlatformId,
    Value,
    Keystate,
    PageLink,
    CreateDate,
    CreateUserId,
    ModifyDate,
    ModifyUserId,
}

#[derive(DeriveIden)]
enum Platform {
    Table,
    Id,
    Name,
    StoreLink,
}

#[derive(Iden, EnumIter)]
pub enum KeyState {
    #[iden = "Unused"]
    Unused,
    #[iden = "Used"]
    Used,
}
