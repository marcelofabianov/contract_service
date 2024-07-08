use crate::{
    grpc::mapper::customer_mapper,
    grpc::pb::customer_pb::{
        customer_service_server::CustomerService as CustomerServiceGrpcTrait,
        CreateCustomerRequest, CreateCustomerResponse,
    },
    internal::application::CustomerService,
    internal::domain::use_case::CreateCustomerInput,
};
use tonic::{Request, Response, Status};

pub struct CustomerController {
    pub service: CustomerService,
}

#[tonic::async_trait]
impl CustomerServiceGrpcTrait for CustomerController {
    async fn create_customer(
        &self,
        request: Request<CreateCustomerRequest>,
    ) -> Result<Response<CreateCustomerResponse>, Status> {
        let req = request.into_inner();

        let input = CreateCustomerInput {
            document: req.document.clone(),
            name: req.name.clone(),
        };

        let output = self.service.create_customer(input).await;

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
