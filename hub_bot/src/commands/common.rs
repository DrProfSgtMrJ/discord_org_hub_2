use crate::Context;
use sea_orm::{DbErr, SqlErr};

pub fn is_unique_violation(db_err: &DbErr) -> bool {
    if let Some(SqlErr::UniqueConstraintViolation(_)) = db_err.sql_err() {
        return true;
    }
    false
}

pub fn get_discord_build_id_from_context(ctx: &Context) -> Option<String> {
    ctx.guild_id().map(|guild_id| guild_id.get().to_string())
}
