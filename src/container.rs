use std::fmt::Error;

use crate::db::Postgres;
use crate::env::Env;
use crate::internal::adapters::CustomerRepositoryPostgres;
use crate::internal::application::CustomerService;
use crate::internal::domain::use_case::CreateCustomerUseCase;
use crate::kafka::Producer;

pub struct Container {
    pub env: Env,
    pub db: Postgres,
    pub producer: Producer,
}

impl Container {
    pub async fn new(
        env: Env,
        db: Postgres,
        producer: Producer,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { env, db, producer })
    }

    pub async fn create_customer_service(&self) -> Result<CustomerService, Error> {
        let pool = self.db.get_pool().await;
        let repository = CustomerRepositoryPostgres::new(pool.clone());
        let use_case = CreateCustomerUseCase::new(repository);

        Ok(CustomerService::new(
            self.env.producer_id,
            self.env.producer_name.clone(),
            self.env.environment.clone(),
            self.producer.clone(),
            use_case,
        ))
    }
}
