use entity::org::{self, ActiveModel as OrgActiveModel, Entity as OrgEntity, Model as OrgModel};
use sea_orm::{ActiveModelTrait, DbErr, EntityTrait, QueryFilter, ColumnTrait};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait OrgService {
    async fn create_org(&self, name: &str, discord_id: &str, owner_id: Uuid) -> Result<OrgModel, DbErr>;
    async fn get_org_by_discord_id(&self, discord_id: &str) -> Result<Option<OrgModel>, DbErr>;
}

#[async_trait::async_trait]
impl OrgService for DbService {
    async fn create_org(&self, name: &str, discord_id: &str, owner_id: Uuid) -> Result<OrgModel, DbErr> {
        let org = OrgActiveModel::new(name, discord_id, owner_id);

        match self.get_connection() {
            Ok(conn) => org.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_org_by_discord_id(&self, discord_id: &str) -> Result<Option<OrgModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                OrgEntity::find().filter(org::Column::DiscordId.eq(discord_id.to_string())).one(conn).await
            }
            Err(err) => Err(err),
        }
    }
}

