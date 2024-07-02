use dotenv::dotenv;
use std::env;

pub struct Env {
    pub environment: String,
    pub database_url: String,
    pub producer_service_id: String,
    pub producer_service_name: String,
}

impl Env {
    pub fn load() -> Self {
        dotenv().ok();

        Env {
            environment: env::var("ENVIRONMENT").expect("ENVIRONMENT must be set"),
            database_url: env::var("DATABASE_URL").expect("DATABASE_URL must be set"),
            producer_service_id: env::var("PRODUCER_SERVICE_ID")
                .expect("PRODUCER_SERVICE_ID must be set"),
            producer_service_name: env::var("PRODUCER_SERVICE_NAME")
                .expect("PRODUCER_SERVICE_NAME must be set"),
        }
    }
}
