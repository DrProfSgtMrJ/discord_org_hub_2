use chrono::NaiveDate;
use entity::{
    sea_orm_active_enums::SeasonType,
    season::{
        self, ActiveModel as SeasonActiveModel, Entity as SeasonEntity, Model as SeasonModel,
    },
};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{DbService, OrderBy};

#[async_trait::async_trait]
pub trait SeasonService {
    async fn create_season(
        &self,
        title: &str,
        org_id: Uuid,
        num_players: i32,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        season_type: Option<SeasonType>,
    ) -> Result<SeasonModel, DbErr>;

    async fn get_season_by_id(&self, season_uuid: &str) -> Result<Option<SeasonModel>, DbErr>;
    async fn get_season_by_uuid(&self, season_uuid: Uuid) -> Result<Option<SeasonModel>, DbErr>;
    async fn get_seasons_by_org_id(
        &self,
        org_id: Uuid,
        order_by: Option<OrderBy<season::Column>>,
    ) -> Result<Vec<SeasonModel>, DbErr>;

    async fn get_latest_season_by_org_id(&self, org_id: Uuid)
    -> Result<Option<SeasonModel>, DbErr>;
}

#[async_trait::async_trait]
impl SeasonService for DbService {
    async fn create_season(
        &self,
        title: &str,
        org_id: Uuid,
        num_players: i32,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
        season_type: Option<SeasonType>,
    ) -> Result<SeasonModel, DbErr> {
        if num_players < 0 {
            return Err(DbErr::Custom(
                "Number of players must be non-negative".to_string(),
            ));
        }

        let season = SeasonActiveModel::new(
            title,
            org_id,
            num_players,
            start_date,
            end_date,
            season_type,
        );

        match self.get_connection() {
            Ok(conn) => season.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_season_by_id(&self, season_uuid: &str) -> Result<Option<SeasonModel>, DbErr> {
        if let Ok(uuid) = season_uuid.parse::<Uuid>() {
            match self.get_connection() {
                Ok(conn) => SeasonEntity::find_by_id(uuid).one(conn).await,
                Err(err) => Err(err),
            }
        } else {
            Err(DbErr::Custom("Invalid UUID format".to_string()))
        }
    }

    async fn get_season_by_uuid(&self, season_uuid: Uuid) -> Result<Option<SeasonModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => SeasonEntity::find_by_id(season_uuid).one(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_seasons_by_org_id(
        &self,
        org_id: Uuid,
        order_by: Option<OrderBy<season::Column>>,
    ) -> Result<Vec<SeasonModel>, DbErr> {
        let conn = self.get_connection()?;
        match order_by {
            Some(OrderBy::Asc { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by(column, sea_orm::Order::Asc)
                    .all(conn)
                    .await
            }
            Some(OrderBy::Desc { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by(column, sea_orm::Order::Desc)
                    .all(conn)
                    .await
            }
            Some(OrderBy::AscNullsFirst { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by_with_nulls(
                        column,
                        sea_orm::Order::Asc,
                        sea_orm::sea_query::NullOrdering::First,
                    )
                    .all(conn)
                    .await
            }
            Some(OrderBy::DescNullsFirst { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by_with_nulls(
                        column,
                        sea_orm::Order::Desc,
                        sea_orm::sea_query::NullOrdering::First,
                    )
                    .all(conn)
                    .await
            }
            Some(OrderBy::AscNullsLast { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by_with_nulls(
                        column,
                        sea_orm::Order::Asc,
                        sea_orm::sea_query::NullOrdering::Last,
                    )
                    .all(conn)
                    .await
            }
            Some(OrderBy::DescNullsLast { column }) => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .order_by_with_nulls(
                        column,
                        sea_orm::Order::Desc,
                        sea_orm::sea_query::NullOrdering::Last,
                    )
                    .all(conn)
                    .await
            }
            None => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .all(conn)
                    .await
            }
        }
    }

    async fn get_latest_season_by_org_id(
        &self,
        org_id: Uuid,
    ) -> Result<Option<SeasonModel>, DbErr> {
        let conn = self.get_connection()?;
        SeasonEntity::find()
            .filter(season::Column::OrgId.eq(org_id))
            .order_by(season::Column::StartDate, sea_orm::Order::Desc)
            .one(conn)
            .await
    }
}
