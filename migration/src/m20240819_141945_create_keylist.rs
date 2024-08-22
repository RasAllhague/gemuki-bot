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
                    .table(Keylist::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Keylist::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Keylist::Name).string_len(50).not_null())
                    .col(ColumnDef::new(Keylist::Description).string_len(255).null())
                    .col(ColumnDef::new(Keylist::OwnerId).big_integer().not_null())
                    .col(ColumnDef::new(Keylist::CreateDate).timestamp().not_null())
                    .col(
                        ColumnDef::new(Keylist::CreateUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Keylist::ModifyDate).timestamp().null())
                    .col(ColumnDef::new(Keylist::ModifyUserId).big_integer().null())
                    .index(
                        Index::create()
                            .name("idx-name-owner")
                            .table(Keylist::Table)
                            .col(Keylist::Name)
                            .col(Keylist::OwnerId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KeylistAccess::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KeylistAccess::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(
                        ColumnDef::new(KeylistAccess::KeylistId)
                            .integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KeylistAccess::TargetUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KeylistAccess::AccessRight)
                            .enumeration(Alias::new("access_right"), AccessRight::iter())
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KeylistAccess::CreateDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KeylistAccess::CreateUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .col(ColumnDef::new(KeylistAccess::ModifyDate).timestamp().null())
                    .col(
                        ColumnDef::new(KeylistAccess::ModifyUserId)
                            .big_integer()
                            .null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeylistAccess::Table, KeylistAccess::KeylistId)
                            .to(Keylist::Table, Keylist::Id),
                    )
                    .index(
                        Index::create()
                            .name("idx-keylist-targetuser")
                            .table(KeylistAccess::Table)
                            .col(KeylistAccess::KeylistId)
                            .col(KeylistAccess::TargetUserId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KeylistKey::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(KeylistKey::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(KeylistKey::KeylistId).integer().not_null())
                    .col(ColumnDef::new(KeylistKey::GamekeyId).integer().not_null())
                    .col(
                        ColumnDef::new(KeylistKey::CreateDate)
                            .timestamp()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(KeylistKey::CreateUserId)
                            .big_integer()
                            .not_null(),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeylistKey::Table, KeylistKey::KeylistId)
                            .to(Keylist::Table, Keylist::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeylistKey::Table, KeylistKey::GamekeyId)
                            .to(GameKey::Table, GameKey::Id),
                    )
                    .index(
                        Index::create()
                            .name("idx-keylist-gamekey")
                            .table(KeylistKey::Table)
                            .col(KeylistKey::KeylistId)
                            .col(KeylistKey::GamekeyId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(KeylistKey::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(KeylistAccess::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Keylist::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum Keylist {
    Table,
    Id,
    Name,
    Description,
    OwnerId,
    CreateDate,
    CreateUserId,
    ModifyDate,
    ModifyUserId,
}

#[derive(DeriveIden)]
enum KeylistAccess {
    Table,
    Id,
    KeylistId,
    TargetUserId,
    AccessRight,
    CreateDate,
    CreateUserId,
    ModifyDate,
    ModifyUserId,
}

#[derive(DeriveIden)]
enum KeylistKey {
    Table,
    Id,
    KeylistId,
    GamekeyId,
    CreateDate,
    CreateUserId,
}

#[derive(Iden, EnumIter)]
pub enum AccessRight {
    #[iden = "Used"]
    Read,
    #[iden = "Used"]
    Write,
    #[iden = "Used"]
    Full,
    #[iden = "Used"]
    Admin,
}

#[derive(DeriveIden)]
enum GameKey {
    Table,
    Id,
}
