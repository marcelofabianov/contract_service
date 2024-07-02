mod db;
mod env;

use db::Postgres;
use env::Env;
use sqlx::query;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let env = Env::load();
    let db = Postgres::new(&env.database_url).await?;

    let row = query!("SELECT 1 + 1 as result")
        .fetch_one(&db.get_pool().await)
        .await?;

    if let Some(result) = row.result {
        println!("1 + 1 = {}", result);
    } else {
        println!("No result found");
    }

    Ok(())
}
