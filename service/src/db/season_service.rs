use std::marker::PhantomData;

use chrono::NaiveDate;
use entity::season::{
    self, ActiveModel as SeasonActiveModel, Entity as SeasonEntity, Model as SeasonModel,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QueryOrder};
use uuid::Uuid;

use crate::{DbService, OrderBy};

impl OrderBy<season::Column> {
    pub fn asc(column: season::Column) -> Self {
        OrderBy::Asc { column }
    }

    pub fn desc(column: season::Column) -> Self {
        OrderBy::Desc { column }
    }
}

#[async_trait::async_trait]
pub trait SeasonService {
    async fn create_season(
        &self,
        title: &str,
        org_id: Uuid,
        num_players: i32,
        start_date: NaiveDate,
        end_date: Option<NaiveDate>,
    ) -> Result<SeasonModel, DbErr>;

    async fn get_season_by_id(&self, season_uuid: &str) -> Result<Option<SeasonModel>, DbErr>;
    async fn get_seasons_by_org_id(
        &self,
        org_id: Uuid,
        order_by: Option<OrderBy<season::Column>>,
    ) -> Result<Vec<SeasonModel>, DbErr>;
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
    ) -> Result<SeasonModel, DbErr> {
        if num_players < 0 {
            return Err(DbErr::Custom(
                "Number of players must be non-negative".to_string(),
            ));
        }

        let season = SeasonActiveModel::new(title, org_id, num_players, start_date, end_date);

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
            None => {
                SeasonEntity::find()
                    .filter(season::Column::OrgId.eq(org_id))
                    .all(conn)
                    .await
            }
        }
    }
}
