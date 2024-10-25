use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(GameKey::Table)
                    .add_column_if_not_exists(big_integer(GameKey::OwnerId).default(0))
                    .to_owned(),
            )
            .await?;

        let update = Query::update()
            .table(GameKey::Table)
            .value(GameKey::OwnerId, Expr::col(GameKey::CreateUserId))
            .to_owned();
        manager.exec_stmt(update).await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let update = Query::update()
            .table(GameKey::Table)
            .value(GameKey::CreateUserId, Expr::col(GameKey::OwnerId))
            .to_owned();
        manager.exec_stmt(update).await?;

        manager
            .alter_table(
                Table::alter()
                    .table(GameKey::Table)
                    .drop_column(GameKey::OwnerId)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
enum GameKey {
    Table,
    CreateUserId,
    OwnerId,
}
