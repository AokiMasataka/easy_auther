use sqlx::{postgres::PgPoolOptions, Pool, Postgres};


pub type Pgsql = Pool<Postgres>;

pub struct DataBaseConfig {
    host: String,
    port: String,
    user: String,
    pass: String,
    db_name: String
}


impl DataBaseConfig {
    pub fn new(
        host: String,
        port: String,
        user: String,
        pass: String,
        db_name: String
    ) -> DataBaseConfig {
        DataBaseConfig{
            host: host,
            port: port,
            user: user,
            pass: pass,
            db_name: db_name
        }
    }

    pub fn get_uri(self: &Self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.user,
            self.pass,
            self.host,
            self.port,
            self.db_name
        )
    }

    pub async fn connection(self: &Self) -> Pgsql {
        println!("connecting to {}", self.get_uri());
        let pool = match PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.get_uri())
            .await {
                Ok(pool) => pool,
                Err(error) => { panic!("postgres connection Error: {}", error); }
            };
            

        println!("connection established!");
        return pool;
    }
}
