use entity::member::{ActiveModel as MemberActiveModel, Model as MemberModel};
use sea_orm::{ActiveModelTrait, DbErr};
use uuid::Uuid;

use crate::DbService;

#[async_trait::async_trait]
pub trait MemberService {
    async fn create_member(&self, user_id: Uuid, org_id: Uuid) -> Result<MemberModel, DbErr>;
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
}
