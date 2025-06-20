use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Game::Table)
                    .add_column_if_not_exists(
                        ColumnDef::new(Game::ImageLink).string_len(500).null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Game::Table)
                    .drop_column(Game::ImageLink)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Game {
    Table,
    ImageLink,
}
