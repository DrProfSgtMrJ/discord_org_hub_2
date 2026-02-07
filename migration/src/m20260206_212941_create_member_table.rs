use crate::m20220101_000001_create_discord_user_table::User;
use crate::m20260206_034522_create_org_table::Org;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Member::Table)
                    .if_not_exists()
                    .col(
                        uuid(Member::Id)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(uuid(Member::UserId).not_null())
                    .col(uuid(Member::OrgId).not_null())
                    .col(boolean(Member::Playing).not_null().default(false))
                    .col(
                        timestamp(Member::CreatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        timestamp(Member::UpdatedAt)
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_member_user_id")
                            .from(Member::Table, Member::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_member_org_id")
                            .from(Member::Table, Member::OrgId)
                            .to(Org::Table, Org::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Member::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Member {
    Table,
    Id,
    UserId,
    OrgId,
    Playing,
    CreatedAt,
    UpdatedAt,
}
