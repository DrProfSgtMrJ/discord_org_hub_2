use crate::Context;
use sea_orm::{DbErr, SqlErr};

pub fn is_unique_violation(db_err: &DbErr) -> bool {
    if let Some(sqlx_err) = db_err.sql_err() {
        match sqlx_err {
            SqlErr::UniqueConstraintViolation(_) => true,
            _ => false,
        }
    } else {
        false
    }
}

pub fn get_discord_build_id_from_context(ctx: &Context) -> Option<String> {
    match ctx.guild_id() {
        Some(guild_id) => Some(guild_id.get().to_string()),
        None => None,
    }
}
