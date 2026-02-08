#[derive(Clone)]
pub struct BaseConfig {
    pub port: u16,
    pub pgsql_host: String,
    pub pgsql_port: String,
    pub pgsql_user: String,
    pub pgsql_pass: String,
    pub pgsql_db: String,
    pub client_id: String,
    pub client_secret: String,
    pub root_email: String,
    pub root_pass: String,
}


fn get_env(var: &str) -> String {
    std::env::var(var).expect(&format!("{} must be set", var))
}

fn get_env_with_defualt(var: &str, value: &str) -> String {
    match std::env::var(var) {
        Ok(v) => v,
        Err(_) => {
            value.to_string()
        }
    }
}


impl BaseConfig {
    pub fn from_env() -> BaseConfig {
        BaseConfig {
            port: get_env("APP_PORT").parse::<u16>().expect("APP_PORT must be a valid u16"),
            pgsql_host: get_env("PGSQL_HOST"),
            pgsql_port: get_env("PGSQL_PORT"),
            pgsql_user: get_env("PGSQL_USER"),
            pgsql_pass: get_env("PGSQL_PASS"),
            pgsql_db: get_env("PGSQL_DB"),
            client_id: get_env("CLIENT_ID"),
            client_secret: get_env("CLIENT_SECRET"),
            root_email: get_env_with_defualt("ROOT_EMAIL", "root"),
            root_pass: get_env_with_defualt("ROOT_PASS", "root")
        }
    }
}
