use entity::discord_user::{self};
use entity::member::{
    self, ActiveModel as MemberActiveModel, Entity as MemberEntity, Model as MemberModel,
};
use entity::org;
use entity::season_member::{ActiveModel as SeasonMemberActiveModel, Model as SeasonMemberModel};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter, QuerySelect, RelationTrait,
};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait MemberService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr>;
    async fn get_members_in_org(&self, org_id: Uuid) -> Result<Vec<MemberModel>, DbErr>;
    async fn get_member_by_ids(
        &self,
        discord_user_id: &str,
        discord_guild_id: &str,
    ) -> Result<Option<MemberModel>, DbErr>;
    async fn add_member_to_season(
        &self,
        member_id: Uuid,
        season_id: Uuid,
        placement: Option<i32>,
    ) -> Result<SeasonMemberModel, DbErr>;
}

#[async_trait::async_trait]
impl MemberService for DbService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr> {
        let member = MemberActiveModel::new(user_id, org_id);

        match self.get_connection() {
            Ok(conn) => member.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_members_in_org(&self, org_id: Uuid) -> Result<Vec<MemberModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                MemberEntity::find()
                    .filter(member::Column::OrgId.eq(org_id))
                    .all(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn get_member_by_ids(
        &self,
        discord_user_id: &str,
        discord_guild_id: &str,
    ) -> Result<Option<MemberModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                member::Entity::find()
                    .join(
                        sea_orm::JoinType::InnerJoin,
                        member::Relation::DiscordUser.def(),
                    )
                    .join(sea_orm::JoinType::InnerJoin, member::Relation::Org.def())
                    .filter(discord_user::Column::DiscordId.eq(discord_user_id))
                    .filter(org::Column::DiscordId.eq(discord_guild_id))
                    .one(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }

    async fn add_member_to_season(
        &self,
        member_id: Uuid,
        season_id: Uuid,
        placement: Option<i32>,
    ) -> Result<SeasonMemberModel, DbErr> {
        let season_member = SeasonMemberActiveModel::new(season_id, member_id, placement);
        match self.get_connection() {
            Ok(conn) => season_member.insert(conn).await,
            Err(err) => Err(err),
        }
    }
}
