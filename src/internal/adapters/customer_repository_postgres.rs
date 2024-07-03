use crate::internal::domain::model::Customer;
use sqlx::PgPool;

pub struct CustomerRepositoryPostgres {
    pool: PgPool,
}

impl CustomerRepositoryPostgres {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn create(&self, customer: Customer) -> Result<Customer, sqlx::Error> {
        let row = sqlx::query!(
            r#"
            INSERT INTO customers (transaction_id, public_id, document, name, disabled_at, created_at, updated_at, deleted_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            RETURNING id, transaction_id, public_id, document, name, disabled_at, created_at, updated_at, deleted_at
            "#,
            customer.transaction_id,
            customer.public_id,
            customer.document,
            customer.name,
            customer.disabled_at,
            customer.created_at,
            customer.updated_at,
            customer.deleted_at
        ).fetch_one(&self.pool).await?;

        Ok(Customer::new(
            row.transaction_id,
            row.id,
            row.public_id,
            row.document,
            row.name,
            row.disabled_at,
            row.created_at,
            row.updated_at,
            row.deleted_at,
        ))
    }
}
