use std::{marker::PhantomData, sync::Arc};

use crate::DbConfig;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr};

pub enum OrderBy<T>
where
    T: sea_orm::IntoSimpleExpr,
{
    Asc { column: T },
    Desc { column: T },
}

#[derive(Debug, Clone)]
pub struct DbService {
    pub config: DbConfig,
    pub connection: Option<Arc<DatabaseConnection>>,
}

impl DbService {
    pub fn from_env() -> Result<Self, DbErr> {
        let config = DbConfig::from_env()?;
        Ok(Self {
            config: config,
            connection: None,
        })
    }

    pub async fn connect(&mut self) -> Result<(), DbErr> {
        let options: ConnectOptions = ConnectOptions::from(&self.config);
        let connection = Arc::new(Database::connect(options).await?);
        self.connection = Some(connection);
        Ok(())
    }

    pub fn get_connection(&self) -> Result<&DatabaseConnection, DbErr> {
        self.connection
            .as_ref()
            .map(|conn| conn.as_ref())
            .ok_or_else(|| DbErr::Custom("Database not connected".to_string()))
    }
}
