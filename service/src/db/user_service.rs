use entity::user::{ActiveModel as UserActiveModel, Model};
use sea_orm::{ActiveModelTrait, DbErr};

use crate::DbService;

#[async_trait::async_trait]
pub trait UserService {
    async fn create_user(
        &self,
        discord_id: &str,
        display_name: &str,
        timezone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Model, DbErr>;
}

#[async_trait::async_trait]
impl UserService for DbService {
    async fn create_user(
        &self,
        discord_id: &str,
        display_name: &str,
        timezone: Option<String>,
        avatar_url: Option<String>,
    ) -> Result<Model, DbErr> {
        let user = UserActiveModel::new(discord_id, display_name, timezone, avatar_url);
        match self.get_connection() {
            Ok(conn) => user.insert(conn).await,
            Err(err) => Err(err),
        }
    }
}
