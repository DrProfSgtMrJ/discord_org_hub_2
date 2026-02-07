use entity::discord_user::{
    self, ActiveModel as DiscordUserActiveModel, Entity as UserEntity, Model as UserModel,
};
use sea_orm::{ActiveModelTrait, ColumnTrait, DbErr, EntityTrait, QueryFilter};

use crate::DbService;

#[async_trait::async_trait]
pub trait UserService {
    async fn create_user(
        &self,
        discord_id: &str,
        display_name: &str,
        timezone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<UserModel, DbErr>;

    async fn get_user_by_discord_id(&self, discord_id: &str) -> Result<Option<UserModel>, DbErr>;
}

#[async_trait::async_trait]
impl UserService for DbService {
    async fn create_user(
        &self,
        discord_id: &str,
        display_name: &str,
        timezone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<UserModel, DbErr> {
        let user = DiscordUserActiveModel::new(discord_id, display_name, timezone, avatar_url);
        match self.get_connection() {
            Ok(conn) => user.insert(conn).await,
            Err(err) => Err(err),
        }
    }

    async fn get_user_by_discord_id(&self, discord_id: &str) -> Result<Option<UserModel>, DbErr> {
        match self.get_connection() {
            Ok(conn) => {
                UserEntity::find()
                    .filter(discord_user::Column::DiscordId.eq(discord_id.to_string()))
                    .one(conn)
                    .await
            }
            Err(err) => Err(err),
        }
    }
}
