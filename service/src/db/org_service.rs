use entity::org::{self, ActiveModel as OrgActiveModel, Entity as OrgEntity, Model as OrgModel};
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DbErr, EntityTrait, IntoActiveModel,
    QueryFilter,
};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait OrgService {
    async fn create_org(
        &self,
        name: &str,
        discord_id: &str,
        owner_id: Uuid,
        current_season_id: Option<Uuid>,
    ) -> Result<OrgModel, DbErr>;
    async fn get_org_by_discord_id(&self, discord_id: &str) -> Result<Option<OrgModel>, DbErr>;
    async fn set_current_season(&self, org_id: Uuid, season_id: Uuid) -> Result<(), DbErr>;
}

#[async_trait::async_trait]
impl OrgService for DbService {
    async fn create_org(
        &self,
        name: &str,
        discord_id: &str,
        owner_id: Uuid,
        current_season_id: Option<Uuid>,
    ) -> Result<OrgModel, DbErr> {
        let org = OrgActiveModel::new(name, discord_id, owner_id, current_season_id);

        match self.get_connection() {
            Ok(conn) => org.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_org_by_discord_id(&self, discord_id: &str) -> Result<Option<OrgModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                OrgEntity::find()
                    .filter(org::Column::DiscordId.eq(discord_id.to_string()))
                    .one(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn set_current_season(&self, org_id: Uuid, season_id: Uuid) -> Result<(), DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                if let Some(org) = OrgEntity::find_by_id(org_id).one(conn).await? {
                    let mut org_active: OrgActiveModel = org.into_active_model();
                    org_active.current_season_id = Set(Some(season_id));
                    org_active.save(conn).await?;
                    Ok(())
                } else {
                    Err(DbErr::RecordNotFound("Org not found".into()))
                }
            }
            Err(err) => Err(err),
        }
    }
}
