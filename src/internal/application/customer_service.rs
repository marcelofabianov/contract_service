use crate::internal::domain::event::{CustomerCreatedEvent, CustomerCreatedEventPayload};
use crate::internal::domain::model::Customer;
use crate::internal::domain::use_case::{CreateCustomerInput, CreateCustomerUseCase};
use crate::kafka::Producer;
use std::error::Error;
use uuid::Uuid;

pub struct CustomerService {
    pub producer_id: Uuid,
    pub producer_name: String,
    pub environment: String,
    pub producer: Producer,
    pub create_customer_use_case: CreateCustomerUseCase,
}

impl CustomerService {
    pub fn new(
        producer_id: Uuid,
        producer_name: String,
        environment: String,
        producer: Producer,
        create_customer_use_case: CreateCustomerUseCase,
    ) -> Self {
        Self {
            producer_id,
            producer_name,
            environment,
            producer,
            create_customer_use_case,
        }
    }

    pub async fn create_customer(
        &self,
        input: CreateCustomerInput,
    ) -> Result<Customer, Box<dyn Error>> {
        let output = self.create_customer_use_case.execute(input).await?;

        let customer = &output.customer;

        let customer_created = CustomerCreatedEvent::new(
            self.producer_id,
            self.producer_name.clone(),
            CustomerCreatedEventPayload {
                transaction_id: customer.transaction_id,
                id: customer.id,
                public_id: customer.public_id,
                document: customer.document.clone(),
                name: customer.name.clone(),
                disabled_at: customer.disabled_at,
                created_at: customer.created_at,
                updated_at: customer.updated_at,
                deleted_at: customer.deleted_at,
            },
            self.environment.clone(),
        );

        let data_json = customer_created.to_json()?;
        // publicar evento no kafka
        println!("{}", data_json);

        Ok(customer.clone())
    }
}
