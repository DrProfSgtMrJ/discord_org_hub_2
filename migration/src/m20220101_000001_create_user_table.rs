use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(
                        uuid(User::Id)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(User::DiscordId).not_null().unique_key())
                    .col(string(User::DisplayName).not_null())
                    .col(string(User::Timezone).not_null().default("UTC"))
                    .col(string(User::AvatarUrl).not_null().default(""))
                    .col(
                        timestamp(User::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(User::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("inx_user_discord_id")
                    .table(User::Table)
                    .col(User::DiscordId)
                    .unique()
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Drop Index first
        manager
            .drop_index(Index::drop().name("idx_user_discord_id").to_owned())
            .await?;
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    DisplayName,
    DiscordId,
    Timezone,
    AvatarUrl,
    CreatedAt,
    UpdatedAt,
}
