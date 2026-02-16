use std::sync::Arc;

use crate::DbConfig;

use sea_orm::{ConnectOptions, Database, DatabaseConnection, DbErr, IntoSimpleExpr};

pub enum OrderBy<C>
where
    C: IntoSimpleExpr,
{
    Asc { column: C },
    Desc { column: C },
    AscNullsFirst { column: C },
    DescNullsFirst { column: C },
    AscNullsLast { column: C },
    DescNullsLast { column: C },
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
            config,
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
