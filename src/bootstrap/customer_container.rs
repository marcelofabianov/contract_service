use crate::db::Postgres;
use crate::env::Env;
use crate::internal::adapters::CustomerRepositoryPostgres;
use crate::internal::application::CustomerService;
use crate::internal::domain::use_case::CreateCustomerUseCase;

pub struct CustomerContainer {
    pub env: Env,
    pub db: Postgres,
}

impl CustomerContainer {
    pub async fn new(env: Env, db: Postgres) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self { env, db })
    }

    pub async fn create_customer_service(&self) -> CustomerService {
        let pool = self.db.get_pool().await;
        let repository = CustomerRepositoryPostgres::new(pool.clone());
        let use_case = CreateCustomerUseCase::new(repository);

        CustomerService::new(
            use_case,
            self.env.producer_id,
            self.env.producer_name.clone(),
            self.env.environment.clone(),
        )
    }
}
