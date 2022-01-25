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

#[cfg(test)]
mod tests {
    use std::time::Instant;

    use serde::Serialize;
    use tera::Tera;

    use super::*;

    #[derive(Serialize)]
    pub struct Data {
        pub elapsed_days: String,
        pub appid: String,
        pub codebase: String,
        pub run: String,
        pub metadata_size: String,
        pub sha256: String,
        pub fp: String,
        pub hash_sha256: String,
    }

    #[test]
    fn test_name() {
        let mut ctx = tera::Context::new();
        let templates = Tera::new("templates/**/*.xml").unwrap();
        let data = Data {
            elapsed_days: "1".to_string(),
            appid: "2".to_string(),
            codebase: "3".to_string(),
            run: "4".to_string(),
            metadata_size: "5".to_string(),
            sha256: "6".to_string(),
            fp: "7".to_string(),
            hash_sha256: "8".to_string(),
        };

        let start = Instant::now();
        ctx.insert("data", &data);

        let body = templates.render("update.xml", &ctx).unwrap();

        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);
        println!("{:?}", body);
    }
}
