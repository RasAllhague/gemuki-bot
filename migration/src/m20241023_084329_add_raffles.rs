use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(KeyRaffle::Table)
                    .if_not_exists()
                    .col(pk_auto(KeyRaffle::Id))
                    .col(string_len(KeyRaffle::Name, 50))
                    .col(string_len_null(KeyRaffle::Description, 255))
                    .col(string_len_null(KeyRaffle::ImageLink, 255))
                    .col(big_integer(KeyRaffle::OwnerId))
                    .col(timestamp_null(KeyRaffle::StartAt))
                    .col(timestamp_null(KeyRaffle::EndAt))
                    .col(integer_null(KeyRaffle::DurationInSeconds))
                    .col(integer_null(KeyRaffle::PossibleWinners))
                    .col(timestamp(KeyRaffle::CreateDate))
                    .col(big_integer(KeyRaffle::CreateUserId))
                    .col(timestamp_null(KeyRaffle::ModifyDate))
                    .col(big_integer_null(KeyRaffle::ModifyUserId))
                    .index(
                        Index::create()
                            .name("idx-name-owner")
                            .table(KeyRaffle::Table)
                            .col(KeyRaffle::Name)
                            .col(KeyRaffle::OwnerId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KeyRaffleKey::Table)
                    .if_not_exists()
                    .col(pk_auto(KeyRaffleKey::Id))
                    .col(integer(KeyRaffleKey::KeyRaffleId))
                    .col(integer(KeyRaffleKey::GamekeyId))
                    .col(timestamp(KeyRaffleKey::CreateDate))
                    .col(big_integer(KeyRaffleKey::CreateUserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeyRaffleKey::Table, KeyRaffleKey::KeyRaffleId)
                            .to(KeyRaffle::Table, KeyRaffle::Id),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeyRaffleKey::Table, KeyRaffleKey::GamekeyId)
                            .to(GameKey::Table, GameKey::Id),
                    )
                    .index(
                        Index::create()
                            .name("idx-raffle-gamekey")
                            .table(KeyRaffle::Table)
                            .col(KeyRaffleKey::KeyRaffleId)
                            .col(KeyRaffleKey::GamekeyId)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(KeyRaffleWinner::Table)
                    .if_not_exists()
                    .col(pk_auto(KeyRaffleWinner::Id))
                    .col(big_integer(KeyRaffleWinner::WinnerId))
                    .col(integer(KeyRaffleWinner::KeyRaffleKeyId))
                    .col(timestamp(KeyRaffleWinner::CreateDate))
                    .col(big_integer(KeyRaffleWinner::CreateUserId))
                    .foreign_key(
                        ForeignKey::create()
                            .from(KeyRaffleWinner::Table, KeyRaffleWinner::KeyRaffleKeyId)
                            .to(KeyRaffle::Table, KeyRaffle::Id),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(KeyRaffleWinner::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(KeyRaffleKey::Table)
                    .if_exists()
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(KeyRaffle::Table).if_exists().to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum KeyRaffle {
    Table,
    Id,
    Name,
    Description,
    ImageLink,
    OwnerId,
    StartAt,
    EndAt,
    DurationInSeconds,
    PossibleWinners,
    CreateDate,
    CreateUserId,
    ModifyDate,
    ModifyUserId,
}

#[derive(DeriveIden)]
enum KeyRaffleKey {
    Table,
    Id,
    KeyRaffleId,
    GamekeyId,
    CreateDate,
    CreateUserId,
}

#[derive(DeriveIden)]
enum KeyRaffleWinner {
    Table,
    Id,
    WinnerId,
    KeyRaffleKeyId,
    CreateDate,
    CreateUserId,
}

#[derive(DeriveIden)]
enum GameKey {
    Table,
    Id,
}
