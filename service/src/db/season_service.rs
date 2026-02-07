use chrono::NaiveDate;
use entity::season::{
    self, ActiveModel as SeasonActiveModel, Entity as SeasonEntity, Model as SeasonModel,
};

use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::DbService;

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
}
