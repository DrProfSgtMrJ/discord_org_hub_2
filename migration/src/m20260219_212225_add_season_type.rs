use sea_orm_migration::{
    prelude::{extension::postgres::Type, *},
    schema::*,
    sea_orm::{EnumIter, Iterable},
};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .create_type(
                Type::create()
                    .as_enum(SeasonType::Type)
                    .values(SeasonType::iter())
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .add_column(
                        enumeration(SeasonType::Type, Season::SeasonType, SeasonType::iter())
                            .not_null()
                            .default(SeasonType::Survivor.to_string()),
                    )
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Replace the sample below with your own migration scripts
        manager
            .alter_table(
                Table::alter()
                    .table(Season::Table)
                    .drop_column(Season::SeasonType)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_type(Type::drop().name(SeasonType::Type).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Season {
    Table,
    SeasonType,
}

#[derive(DeriveIden, EnumIter)]
enum SeasonType {
    #[sea_orm(iden = "season_type")]
    Type,
    #[sea_orm(iden = "Survivor")]
    Survivor,
    #[sea_orm(iden = "Traitors")]
    Traitors,
    #[sea_orm(iden = "BigBrother")]
    BigBrother,
    #[sea_orm(iden = "TheChallenge")]
    TheChallenge,
    #[sea_orm(iden = "Other")]
    Other,
}
