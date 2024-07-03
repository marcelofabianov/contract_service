mod db;
mod env;
mod internal {
    pub mod adapters;
    pub mod domain {
        pub mod event;
        pub mod model;
        pub mod use_case;
    }
}

use db::Postgres;
use env::Env;
use internal::adapters::CustomerRepositoryPostgres;
use internal::domain::event::{CustomerCreatedEvent, CustomerCreatedEventPayload};
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

    let output = use_case.execute(input).await?;

    let customer = output.customer;

    let customer_created = CustomerCreatedEvent::new(
        env.producer_id,
        env.producer_name,
        CustomerCreatedEventPayload {
            transaction_id: customer.transaction_id,
            id: customer.id,
            public_id: customer.public_id,
            document: customer.document,
            name: customer.name,
            disabled_at: customer.disabled_at,
            created_at: customer.created_at,
            updated_at: customer.updated_at,
            deleted_at: customer.deleted_at,
        },
        env.environment,
    );

    let data_json = customer_created.to_json();

    if let Ok(json) = data_json {
        println!("{}", json);
    } else {
        println!("Error converting to JSON");
    }

    Ok(())
}
