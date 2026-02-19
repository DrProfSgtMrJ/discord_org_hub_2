pub use sea_orm_migration::prelude::*;

mod m20220101_000001_create_discord_user_table;
mod m20260206_034522_create_org_table;
mod m20260206_212941_create_member_table;
mod m20260207_023556_create_season_table;
mod m20260207_033742_remove_playing_from_member;
mod m20260207_081033_change_num_players_to_unsigned;
mod m20260219_212225_add_season_type;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_create_discord_user_table::Migration),
            Box::new(m20260206_034522_create_org_table::Migration),
            Box::new(m20260206_212941_create_member_table::Migration),
            Box::new(m20260207_023556_create_season_table::Migration),
            Box::new(m20260207_033742_remove_playing_from_member::Migration),
            Box::new(m20260207_081033_change_num_players_to_unsigned::Migration),
            Box::new(m20260219_212225_add_season_type::Migration),
        ]
    }
}
