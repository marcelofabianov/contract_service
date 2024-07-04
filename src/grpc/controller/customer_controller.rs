use chrono::Utc;
use prost_types::Timestamp;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::bootstrap::CustomerContainer;
use crate::grpc::service::customer_grpc_service::{
    customer_service_server::CustomerService as CustomerServiceGrpcTrait, CreateCustomerRequest,
    CreateCustomerResponse, Customer,
};

#[derive(Debug, Default)]
pub struct CustomerController {
    pub container: CustomerContainer,
}

#[tonic::async_trait]
impl CustomerServiceGrpcTrait for CustomerController {
    async fn create_customer(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<CreateCustomerResponse>, Status> {
        let req = request.into_inner();

        //... operacao de create pelo customer_service

        //... customer sera obtido de um use case
        let customer = Customer {
            transaction_id: Uuid::new_v4().to_string(),
            id: 0,
            public_id: Uuid::new_v4().to_string(),
            document: req.document,
            name: req.name,
            disabled_at: None,
            created_at: Some(Timestamp::from(Utc::now())),
            updated_at: Some(Timestamp::from(Utc::now())),
        };

        let reply = CreateCustomerResponse { customer };

        Ok(Response::new(reply))
    }
}
