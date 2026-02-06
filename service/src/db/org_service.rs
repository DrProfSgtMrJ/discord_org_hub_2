use entity::org::{ActiveModel as OrgActiveModel, Model};
use sea_orm::{ActiveModelTrait, DbErr};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait OrgService {
    async fn create_org(&self, name: &str, discord_id: &str, owner_id: Uuid) -> Result<Model, DbErr>;
}

#[async_trait::async_trait]
impl OrgService for DbService {
    async fn create_org(&self, name: &str, discord_id: &str, owner_id: Uuid) -> Result<Model, DbErr> {
        let org = OrgActiveModel::new(name, discord_id, owner_id);

        match self.get_connection() {
            Ok(conn) => org.insert(conn).await,
            Err(err) => Err(err),
        }
    }
}

