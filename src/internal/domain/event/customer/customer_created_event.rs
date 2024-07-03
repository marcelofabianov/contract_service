use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Error;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerCreatedEvent {
    pub topic: String,
    pub producer_id: Uuid,
    pub producer_name: String,
    pub transaction_id: Uuid,
    pub timestamp: String,
    pub event_type: String,
    pub payload: CustomerCreatedEventPayload,
    pub metadata: CustomerCreatedEventMetadata,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerCreatedEventPayload {
    pub transaction_id: Uuid,
    pub id: i32,
    pub public_id: Uuid,
    pub document: String,
    pub name: String,
    pub disabled_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub deleted_at: Option<DateTime<Utc>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CustomerCreatedEventMetadata {
    pub event_id: Uuid,
    pub event_version: String,
    pub event_date: DateTime<Utc>,
    pub environment: String,
}

const EVENT_NAME: &str = "customer.created";
const EVENT_VERSION: &str = "v1";

impl CustomerCreatedEvent {
    pub fn new(
        producer_id: Uuid,
        producer_name: String,
        payload: CustomerCreatedEventPayload,
        environment: String,
    ) -> Self {
        Self {
            // PRODUCER_NAME + EVENT_NAME + EVENT_VERSION + ENVIROMENT
            topic: format!(
                "{}.{}.{}.{}",
                producer_name, EVENT_NAME, EVENT_VERSION, environment
            ),
            producer_id,
            producer_name,
            transaction_id: payload.transaction_id,
            timestamp: Utc::now().timestamp().to_string(),
            event_type: EVENT_NAME.to_string(),
            payload,
            metadata: CustomerCreatedEventMetadata {
                event_id: Uuid::new_v4(),
                event_version: EVENT_VERSION.to_string(),
                event_date: Utc::now(),
                environment: environment,
            },
        }
    }

    pub fn to_json(&self) -> Result<String, Error> {
        serde_json::to_string(&self)
    }
}
