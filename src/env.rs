use dotenv::dotenv;
use std::env;

pub struct Env {
    pub database_url: String,
}

impl Env {
    pub fn load() -> Self {
        dotenv().ok();

        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
        }
    }
}
