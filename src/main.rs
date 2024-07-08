mod container;
mod db;
mod env;
mod error;
mod errors;
mod grpc;
mod internal;
mod kafka;

use error::ServiceError;
use grpc::controller::CustomerController;
use grpc::pb::customer_pb::customer_service_server::CustomerServiceServer;
use grpc::pb::FILE_DESCRIPTOR_SET;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

#[tokio::main]
async fn main() -> Result<(), ServiceError> {
    let env = match env::Env::load() {
        Ok(env) => env,
        Err(err) => return Err(err.into()),
    };

    let db = match db::Postgres::new(&env.database_url).await {
        Ok(db) => db,
        Err(err) => return Err(err.into()),
    };

    let producer = kafka::Producer::new(&env.kafka_brokers);

    let container = container::Container::new(env.clone(), db, producer).await?;
    let customer_service = container.create_customer_service().await?;

    let customer_service = CustomerController {
        service: customer_service,
    };

    let addr: std::net::SocketAddr = "0.0.0.0:50051".parse().unwrap();

    println!("Server listening on {}", addr);

    let reflection_service = ReflectionBuilder::configure()
        .register_encoded_file_descriptor_set(FILE_DESCRIPTOR_SET)
        .build()
        .expect("Failed to create the reflection service");

    Server::builder()
        .add_service(reflection_service)
        .add_service(CustomerServiceServer::new(customer_service))
        .serve(addr)
        .await?;

    Ok(())
}
