use dotenv::dotenv;
use sea_orm::{ConnectOptions, DbErr};

#[derive(Debug, Clone)]
pub struct DbConfig {
    pub connection_url: String,
    pub max_connections: u32,
    pub min_connections: u32,
}

impl DbConfig {
    pub fn from_env() -> Result<Self, DbErr> {
        dotenv().ok();
        let connection_url = std::env::var("DATABASE_URL")
            .map_err(|_| DbErr::Custom("DATABASE_URL must be set".to_string()))?;

        let max_connections = std::env::var("MAX_CONNECTIONS")
            .unwrap_or("10".to_string())
            .parse()
            .map_err(|_| DbErr::Custom("MAX_CONNECTIONS must be a number".to_string()))?;
        let min_connections = std::env::var("MIN_CONNECTIONS")
            .unwrap_or("5".to_string())
            .parse()
            .map_err(|_| DbErr::Custom("MIN_CONNECTIONS must be a number".to_string()))?;
        Ok(Self {
            connection_url,
            max_connections,
            min_connections,
        })
    }
}

impl From<DbConfig> for ConnectOptions {
    fn from(config: DbConfig) -> Self {
        ConnectOptions::new(config.connection_url)
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .to_owned()
    }
}

impl From<&DbConfig> for ConnectOptions {
    fn from(config: &DbConfig) -> Self {
        ConnectOptions::new(config.connection_url.clone())
            .max_connections(config.max_connections)
            .min_connections(config.min_connections)
            .to_owned()
    }
}
