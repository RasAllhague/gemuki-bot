use sea_orm_migration::{
    prelude::*,
    schema::string_len_null,
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                TableAlterStatement::new()
                    .table(Game::Table)
                    .add_column_if_not_exists(string_len_null(Game::ImageLink, 500))
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
