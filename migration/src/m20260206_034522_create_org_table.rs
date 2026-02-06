use sea_orm_migration::{prelude::*, schema::*};
use crate::m20220101_000001_create_discord_user_table::User;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Org::Table)
                    .if_not_exists()
                    .col(uuid(Org::Id)
                        .not_null()
                        .primary_key()
                        .default(Expr::cust("gen_random_uuid()"))
                    )
                    .col(string(Org::Name).not_null())
                    .col(string(Org::DiscordId).not_null().unique_key())
                    .col(uuid(Org::OwnerId).not_null())
                    .col(timestamp(Org::CreatedAt).not_null().default(Expr::current_timestamp()))
                    .col(timestamp(Org::UpdatedAt).not_null().default(Expr::current_timestamp()))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_org_owner_id")
                            .from(Org::Table, Org::OwnerId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::NoAction),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Org::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Org {
    Table,
    Id,
    Name,
    OwnerId,
    DiscordId,
    CreatedAt,
    UpdatedAt,
}
