use crate::Error;
use chrono::NaiveDate;
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

pub fn check_dates(
    start_date: String,
    end_date: Option<String>,
) -> Result<(NaiveDate, Option<NaiveDate>), Error> {
    let valid_fmt = "%Y-%m-%d";
    let start_date = NaiveDate::parse_from_str(&start_date, valid_fmt)
        .map_err(|_| "Invalid Start Date".to_string())?;
    let end_date = match end_date {
        Some(end_date) => {
            let parsed_end_date = NaiveDate::parse_from_str(&end_date, valid_fmt)
                .map_err(|_| "Invalid End Date".to_string())?;

            if start_date > parsed_end_date {
                return Err("Start date must be before end date".into());
            }
            Some(parsed_end_date)
        }
        None => None,
    };
    Ok((start_date, end_date))
}
