use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use crate::state::config::BaseConfig;

pub struct DataBaseConfig {
    host: String,
    port: String,
    user: String,
    pass: String,
    db: String
}

impl DataBaseConfig {
    pub fn from_config(config: &BaseConfig) -> DataBaseConfig{
        DataBaseConfig{
            host: config.pgsql_host.clone(),
            port: config.pgsql_port.clone(),
            user: config.pgsql_user.clone(),
            pass: config.pgsql_pass.clone(),
            db: config.pgsql_db.clone()
        }
    }

    pub fn get_uri(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.pass,
            self.host,
            self.port,
            self.db
        )
    }

    pub async fn connection(&self) -> Result<Pool<Postgres>, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(4)
            .connect(&self.get_uri())
            .await;
        return pool;
    }
}