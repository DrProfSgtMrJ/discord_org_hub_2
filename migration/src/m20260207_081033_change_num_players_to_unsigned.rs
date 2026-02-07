use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SeasonMember::Table)
                    .modify_column(ColumnDef::new(SeasonMember::Placement).unsigned().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .modify_column(ColumnDef::new(Season::NumPlayers).unsigned().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(SeasonMember::Table)
                    .modify_column(ColumnDef::new(SeasonMember::Placement).integer().null())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .modify_column(ColumnDef::new(Season::NumPlayers).integer().not_null())
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum Season {
    Table,
    NumPlayers,
}

#[derive(DeriveIden)]
enum SeasonMember {
    Table,
    Placement,
}
