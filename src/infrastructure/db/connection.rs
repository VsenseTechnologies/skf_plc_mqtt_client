use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn establish_connection(db_url: &str) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect(db_url)
        .await
        .expect("failed to connect to database")
}
