pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_table;
mod m20240722_083119_seeding_data;
mod m20240723_102800_game_image_link;
mod m20240725_122713_game_key_notes;
mod m20250603_114102_key_expiry_date;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_table::Migration),
            Box::new(m20240722_083119_seeding_data::Migration),
            Box::new(m20240723_102800_game_image_link::Migration),
            Box::new(m20240725_122713_game_key_notes::Migration),
            Box::new(m20250603_114102_key_expiry_date::Migration),
        ]
    }
}
