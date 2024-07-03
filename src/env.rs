use dotenv::dotenv;
use std::env;

pub struct Env {
    pub database_url: String,
    pub producer_id: uuid::Uuid,
    pub producer_name: String,
    pub environment: String,
}

impl Env {
    pub fn load() -> Self {
        dotenv().ok();

        Env {
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            producer_id: uuid::Uuid::parse_str(
                &env::var("PRODUCER_ID").expect("PRODUCER_ID must be set"),
            )
            .expect("PRODUCER_ID must be a valid UUID"),
            producer_name: env::var("PRODUCER_NAME").expect("PRODUCER_NAME must be set"),
            environment: env::var("ENVIRONMENT").expect("ENVIRONMENT must be set"),
        }
    }
}
