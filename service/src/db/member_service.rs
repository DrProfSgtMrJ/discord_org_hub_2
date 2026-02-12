use entity::member::{
    self, ActiveModel as MemberActiveModel, Entity as MemberEntity, Model as MemberModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait MemberService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr>;
    async fn get_members_in_org(&self, org_id: Uuid) -> Result<Vec<MemberModel>, DbErr>;
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
}
