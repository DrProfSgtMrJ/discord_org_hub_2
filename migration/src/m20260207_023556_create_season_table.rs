use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts

        manager
            .create_table(
                Table::create()
                    .table(Season::Table)
                    .if_not_exists()
                    .col(
                        uuid(Season::Id)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(string(Season::Title).not_null())
                    .col(uuid(Season::OrgId).not_null())
                    .col(integer(Season::NumPlayers).not_null())
                    .col(date(Season::StartDate).not_null())
                    .col(date_null(Season::EndDate))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_season_org_id")
                            .from(Season::Table, Season::OrgId)
                            .to(Org::Table, Org::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_table(
                Table::create()
                    .table(SeasonMember::Table)
                    .if_not_exists()
                    .col(
                        uuid(SeasonMember::Id)
                            .not_null()
                            .primary_key()
                            .default(Expr::cust("gen_random_uuid()")),
                    )
                    .col(uuid(SeasonMember::SeasonId).not_null())
                    .col(uuid(SeasonMember::MemberId).not_null())
                    .col(integer_null(SeasonMember::Placement))
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_season_member_season_id")
                            .from(SeasonMember::Table, SeasonMember::SeasonId)
                            .to(Season::Table, Season::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_season_member_member_id")
                            .from(SeasonMember::Table, SeasonMember::MemberId)
                            .to(Member::Table, Member::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Org::Table)
                    .add_column(uuid_null(Org::CurrentSeasonId))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_org_current_season_id")
                    .from(Org::Table, Org::CurrentSeasonId)
                    .to(Season::Table, Season::Id)
                    .on_delete(ForeignKeyAction::SetNull)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_season_member_season_id")
                    .table(SeasonMember::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_season_member_member_id")
                    .table(SeasonMember::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_org_current_season_id")
                    .table(Org::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Org::Table)
                    .drop_column(Org::CurrentSeasonId)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(SeasonMember::Table).to_owned())
            .await?;

        manager
            .drop_table(Table::drop().table(Season::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Season {
    Table,
    Id,
    Title,
    OrgId,
    NumPlayers,
    StartDate,
    EndDate,
}

#[derive(DeriveIden)]
pub enum SeasonMember {
    Table,
    Id,
    SeasonId,
    MemberId,
    Placement,
}

#[derive(DeriveIden)]
pub enum Org {
    Table,
    Id,
    CurrentSeasonId,
}

#[derive(DeriveIden)]
pub enum Member {
    Table,
    Id,
}
