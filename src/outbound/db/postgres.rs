use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    Pool, Postgres,
};

use crate::config::DatabaseSettings;

#[derive(Debug, Clone)]
pub struct Pg {
    pub pool: Pool<Postgres>,
}

impl Pg {
    pub async fn new(config: DatabaseSettings) -> anyhow::Result<Self, anyhow::Error> {
        let opts = PgConnectOptions::new()
            .host(&config.host)
            .port(config.port)
            .username(&config.username)
            .password(&config.password)
            .database(&config.database_name);
        let pool = PgPoolOptions::new().connect_with(opts).await?;
        Ok(Self { pool })
    }
}
