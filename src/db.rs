use sqlx::{postgres::PgPoolOptions, PgPool};

pub struct Postgres {
    pool: PgPool,
}

impl Postgres {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub async fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
