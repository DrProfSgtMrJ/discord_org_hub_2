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
