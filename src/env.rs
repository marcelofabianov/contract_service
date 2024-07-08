use dotenv::dotenv;
use std::env;
use std::error::Error;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Env {
    pub database_url: String,
    pub producer_id: uuid::Uuid,
    pub producer_name: String,
    pub environment: String,
    pub kafka_brokers: String,
}

impl Env {
    pub fn load() -> Result<Self, Box<dyn Error>> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")?;
        let producer_id = Uuid::parse_str(&env::var("PRODUCER_ID")?)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let producer_name = env::var("PRODUCER_NAME")?;
        let environment = env::var("ENVIRONMENT")?;
        let kafka_brokers = env::var("KAFKA_BROKERS")?;

        Ok(Env {
            database_url,
            producer_id,
            producer_name,
            environment,
            kafka_brokers,
        })
    }
}
