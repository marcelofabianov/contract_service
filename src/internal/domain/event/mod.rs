pub mod customer {
    pub mod customer_created_event;
}

pub use customer::customer_created_event::{CustomerCreatedEvent, CustomerCreatedEventPayload};
