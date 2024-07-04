use chrono::Utc;
use prost_types::Timestamp;
use tonic::{Request, Response, Status};
use uuid::Uuid;

use crate::bootstrap::CustomerContainer;
use crate::grpc::pb::customer_pb::{
    customer_service_server::CustomerService as CustomerServiceGrpcTrait, CreateCustomerRequest,
    CreateCustomerResponse, Customer,
};

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

        // Simulando operação de criação pelo serviço de cliente
        // Suponha que você tenha lógica aqui para criar um cliente

        // Simulando obtenção do cliente de um caso de uso
        let customer = Customer {
            transaction_id: Uuid::new_v4().to_string(),
            id: 0,
            public_id: Uuid::new_v4().to_string(),
            document: req.document,
            name: req.name,
            disabled_at: None, // Aqui você precisa usar `None` porque é opcional
            created_at: Some(Timestamp {
                seconds: Utc::now().timestamp(),
                nanos: 0,
            }),
            updated_at: Some(Timestamp {
                seconds: Utc::now().timestamp(),
                nanos: 0,
            }),
        };

        let reply = CreateCustomerResponse {
            customer: Some(customer),
        };

        Ok(Response::new(reply))
    }
}
