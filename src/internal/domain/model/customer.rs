use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Customer {
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

impl Customer {
    pub fn new(
        transaction_id: Uuid,
        id: i32,
        public_id: Uuid,
        document: String,
        name: String,
        disabled_at: Option<DateTime<Utc>>,
        created_at: DateTime<Utc>,
        updated_at: DateTime<Utc>,
        deleted_at: Option<DateTime<Utc>>,
    ) -> Self {
        Self {
            transaction_id,
            id,
            public_id,
            document,
            name,
            disabled_at,
            created_at,
            updated_at,
            deleted_at,
        }
    }
}
