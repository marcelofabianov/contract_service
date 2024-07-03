mod db;
mod env;
mod internal {
    pub mod adapters;
    pub mod domain {
        pub mod model;
    }
}

use chrono::Utc;
use db::Postgres;
use env::Env;
use internal::adapters::CustomerRepositoryPostgres;
use internal::domain::model::Customer;
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::load();
    let db = Postgres::new(&env.database_url).await?;
    let pool = db.get_pool().await;

    let repository = CustomerRepositoryPostgres::new(pool);

    let new_customer = Customer {
        transaction_id: Uuid::new_v4(),
        id: 0,
        public_id: Uuid::new_v4(),
        document: "344353598989".to_string(),
        name: "Rust Foundation".to_string(),
        disabled_at: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
        deleted_at: None,
    };

    let customer = repository.create(new_customer).await?;

    println!("{:?}", customer);

    Ok(())
}
