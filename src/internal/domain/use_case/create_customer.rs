use crate::internal::adapters::CustomerRepositoryPostgres;
use crate::internal::domain::model::Customer;
use chrono::Utc;
use std::error::Error;
use uuid::Uuid;

pub struct CreateCustomerUseCase {
    pub repository: CustomerRepositoryPostgres,
}

pub struct CreateCustomerInput {
    pub document: String,
    pub name: String,
}

pub struct CreateCustomerOutput {
    pub customer: Customer,
}

impl CreateCustomerUseCase {
    pub fn new(repository: CustomerRepositoryPostgres) -> Self {
        Self { repository }
    }

    pub async fn execute(
        &self,
        input: CreateCustomerInput,
    ) -> Result<CreateCustomerOutput, Box<dyn Error>> {
        let customer = Customer::new(
            Uuid::new_v4(),
            0,
            Uuid::new_v4(),
            input.document,
            input.name,
            None,
            Utc::now(),
            Utc::now(),
            None,
        );

        let customer = self.repository.create(customer).await?;

        Ok(CreateCustomerOutput { customer })
    }
}
