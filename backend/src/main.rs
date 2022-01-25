use sea_orm::Database;
use std::env;

mod entity;
mod omaha;
mod setup;

#[tokio::main]
async fn main() {
    env::set_var("RUST_LOG", "debug");
    tracing_subscriber::fmt::init();

    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let host = env::var("HOST").expect("HOST is not set in .env file");
    let port = env::var("PORT").expect("PORT is not set in .env file");
    let server_url = format!("{}:{}", host, port);

    let conn = Database::connect(db_url)
        .await
        .expect("Database connection failed");

    let _ = setup::create_post_table(&conn).await;
    let _ = setup::create_user_table(&conn).await;
    let _ = setup::create_payload_table(&conn).await;
}
