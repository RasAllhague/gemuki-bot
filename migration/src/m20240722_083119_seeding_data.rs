use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Platform::Table)
            .columns([Platform::Name, Platform::StoreLink])
            .values_panic(["Steam".into(), "https://store.steampowered.com/".into()])
            .values_panic(["Epic".into(), "https://store.epicgames.com/de/".into()])
            .values_panic(["Ubisoft".into(), "https://www.ubisoft.com/de-de/ubisoft-connect".into()])
            .values_panic(["EA Play".into(), "https://www.ea.com/de-de/ea-play".into()])
            .to_owned();

        manager.exec_stmt(insert).await
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

#[derive(DeriveIden)]
enum Platform {
    Table,
    Name,
    StoreLink,
}