mod db;
mod env;
mod internal {
    pub mod adapters;
    pub mod domain {
        pub mod event;
        pub mod model;
        pub mod use_case;
    }
    pub mod application;
}

use db::Postgres;
use env::Env;
use internal::adapters::CustomerRepositoryPostgres;
use internal::application::CustomerService;
use internal::domain::use_case::{CreateCustomerInput, CreateCustomerUseCase};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::load();
    let db = Postgres::new(&env.database_url).await?;
    let pool = db.get_pool().await;

    let repository = CustomerRepositoryPostgres::new(pool);

    let use_case = CreateCustomerUseCase::new(repository);

    let input = CreateCustomerInput {
        document: "12345678901".to_string(),
        name: "Rust Foundation".to_string(),
    };

    let service = CustomerService::new(
        use_case,
        env.producer_id,
        env.producer_name,
        env.environment,
    );

    let customer = service.create_customer(input).await?;

    println!("{:?}", customer);

    Ok(())
}
