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
                    .table(Keylist::Table)
                    .if_not_exists()
                    .col(pk_auto(Keylist::Id))
                    .col(string_len(Keylist::Name, 50))
                    .col(string_len_null(Keylist::Description, 255))
                    .col(big_integer(Keylist::OwnerId))
                    .col(timestamp(Keylist::CreateDate))
                    .col(big_integer(Keylist::CreateUserId))
                    .col(timestamp_null(Keylist::ModifyDate))
                    .col(big_integer_null(Keylist::ModifyUserId))
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
                    .col(pk_auto(KeylistAccess::Id))
                    .col(integer(KeylistAccess::KeylistId))
                    .col(big_integer(KeylistAccess::TargetUserId))
                    .col(enumeration(
                        KeylistAccess::AccessRight,
                        Alias::new("access_right"),
                        AccessRight::iter(),
                    ))
                    .col(timestamp(KeylistAccess::CreateDate))
                    .col(big_integer(KeylistAccess::CreateUserId))
                    .col(timestamp_null(KeylistAccess::ModifyDate))
                    .col(big_integer_null(KeylistAccess::ModifyUserId))
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
                    .col(pk_auto(KeylistKey::Id))
                    .col(integer(KeylistKey::KeylistId))
                    .col(integer(KeylistKey::GamekeyId))
                    .col(timestamp(KeylistKey::CreateDate))
                    .col(big_integer(KeylistKey::CreateUserId))
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
