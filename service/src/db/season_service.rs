use chrono::NaiveDate;
use entity::season::{
    ActiveModel as SeasonActiveModel, Entity as SeasonEntity, Model as SeasonModel,
};

use sea_orm::{ActiveModelTrait, DbErr, EntityTrait};
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

    async fn get_season_by_id(&self, season_uuid: &str) -> Result<Option<SeasonModel>, DbErr>;
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
}
