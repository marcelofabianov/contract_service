mod bootstrap;
mod db;
mod env;
mod grpc;
mod internal;

use grpc::controller::CustomerController;
use grpc::pb::customer_pb::customer_service_server::CustomerServiceServer;
use grpc::pb::FILE_DESCRIPTOR_SET;
use tonic::transport::Server;
use tonic_reflection::server::Builder as ReflectionBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = env::Env::load();
    let db = db::Postgres::new(&env.database_url).await?;

    let container = bootstrap::CustomerContainer::new(env, db).await?;

    let addr: std::net::SocketAddr = "[::1]:50051".parse().unwrap();

    let customer_service = CustomerController { container };

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
