use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_index(
                Index::create()
                    .name("idx_season_member_unique")
                    .table(SeasonMember::Table)
                    .col(SeasonMember::SeasonId)
                    .col(SeasonMember::MemberId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .drop_index(
                Index::drop()
                    .name("idx_season_member_unique")
                    .table(SeasonMember::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum SeasonMember {
    Table,
    SeasonId,
    MemberId,
}
