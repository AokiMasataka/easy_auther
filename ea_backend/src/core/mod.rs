use sqlx::{Pool, Postgres};
use jwt_simple::prelude::RS256KeyPair;

pub mod config;
pub mod database;
pub mod exceptions;


#[derive(Clone)]
pub struct AppState {
    pub config: config::BaseConfig,
    pub db_pool: Pool<Postgres>,
    pub key_pair: RS256KeyPair,
}



impl AppState {
    pub async fn from_env() -> Result<AppState, sqlx::Error> {
        let config = config::BaseConfig::from_env();
        let db_pool = match database::DataBaseConfig::from_config(&config)
            .connection()
            .await {
                Ok(pool) => pool,
                Err(error) => { return Err(error); }
            };
        let key_pair = RS256KeyPair::generate(2048).unwrap();

        Ok(AppState{config, db_pool, key_pair})
    }    
}
