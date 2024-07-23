use sea_orm::{EnumIter, Iterable};
use sea_orm_migration::prelude::*;

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
                    .col(
                        ColumnDef::new(Game::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Game::Title).string_len(255).not_null())
                    .col(ColumnDef::new(Game::Description).string_len(500).null())
                    .col(ColumnDef::new(Game::CreateDate).timestamp().not_null())
                    .col(ColumnDef::new(Game::CreateUserId).big_integer().not_null())
                    .col(ColumnDef::new(Game::ModifyDate).timestamp().null())
                    .col(ColumnDef::new(Game::ModifyUserId).big_integer().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(GameKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(GameKey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(GameKey::GameId).integer().not_null())
                    .col(ColumnDef::new(GameKey::PlatformId).integer().null())
                    .col(ColumnDef::new(GameKey::Value).string_len(255).not_null())
                    .col(
                        ColumnDef::new(GameKey::Keystate)
                            .enumeration(Alias::new("keystate"), KeyState::iter())
                            .not_null(),
                    )
                    .col(ColumnDef::new(GameKey::PageLink).string_len(500).null())
                    .col(ColumnDef::new(GameKey::CreateDate).timestamp().not_null())
                    .col(
                        ColumnDef::new(GameKey::CreateUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(GameKey::ModifyDate).timestamp().null())
                    .col(ColumnDef::new(GameKey::ModifyUserId).big_integer().null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_table(
                Table::create()
                    .table(Platform::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Platform::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Platform::Name).string_len(255).not_null())
                    .col(ColumnDef::new(Platform::StoreLink).string_len(500).null())
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
