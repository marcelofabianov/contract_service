mod bootstrap;
mod db;
mod env;
mod grpc {
    pub mod controller;
    pub mod service {
        tonic::include_proto!("customer_grpc_service");
    }
}
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
use grpc::controller::CustomerController;
use grpc::service::customer_service_server::CustomerServiceServer;
use tonic::transport::Server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::load();
    let db = Postgres::new(&env.database_url).await?;

    let container = CustomerContainer::new(env, db).await?;

    let addr = "[::1]:50051".parse().unwrap();
    let customer_service = CustomerController { container };

    println!("Server listening on {}", addr);

    Server::builder()
        .add_service(CustomerServiceServer::new(customer_service))
        .serve(addr)
        .await?;

    Ok(())
}
