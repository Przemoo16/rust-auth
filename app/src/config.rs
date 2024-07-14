use std::env::var;

pub struct Config {
    pub database_url: String,
    pub database_pool_max_connections: u32,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: read_env("DATABASE_URL"),
            database_pool_max_connections: read_env("DATABASE_POOL_MAX_CONNECTIONS")
                .parse()
                .expect("DATABASE_POOL_MAX_CONNECTIONS must be a number"),
        }
    }
}

fn read_env(key: &str) -> String {
    var(key).expect(&format!("Couldn't read the {} env variable", key))
}
