mod bootstrap;
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

use bootstrap::CustomerContainer;
use db::Postgres;
use env::Env;
use internal::domain::use_case::CreateCustomerInput;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::load();
    let db = Postgres::new(&env.database_url).await?;

    let container = CustomerContainer::new(env, db).await?;
    let service = container.create_customer_service().await;

    let input = CreateCustomerInput {
        document: "12345678901".to_string(),
        name: "Rust Foundation".to_string(),
    };

    let customer = service.create_customer(input).await?;

    println!("{:?}", customer);

    Ok(())
}
