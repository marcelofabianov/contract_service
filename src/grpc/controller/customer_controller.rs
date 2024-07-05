use tonic::{Request, Response, Status};

use crate::bootstrap::CustomerContainer;
use crate::grpc::pb::customer_pb::{
    customer_service_server::CustomerService as CustomerServiceGrpcTrait, CreateCustomerRequest,
    CreateCustomerResponse,
};

use crate::grpc::mapper::customer_mapper;
use crate::internal::domain::use_case::CreateCustomerInput;

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

        let service = self.container.create_customer_service().await;

        let input = CreateCustomerInput {
            document: req.document.clone(),
            name: req.name.clone(),
        };

        let output = service.create_customer(input).await;

        match output {
            Ok(customer) => {
                let customer_grpc = customer_mapper::map_to_customer_grpc(customer);
                let reply = CreateCustomerResponse {
                    customer: Some(customer_grpc),
                };
                Ok(Response::new(reply))
            }
            Err(err) => {
                println!("Failed to create customer: {:?}", err);
                Err(Status::unknown("Failed to create customer"))
            }
        }
    }
}
