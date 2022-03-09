use std::{env, net::SocketAddr};

use axum::{extract::Extension, routing::get, Router};
use error::AppError;
use sea_orm::{Database, DatabaseConnection};
mod ar;
mod error;
mod service;
mod setup;

#[tokio::main]
async fn main() -> Result<(), AppError> {
    // 设置日志，测试等级为debug
    env::set_var("RUST_LOG", "debug");
    // 获取环境参数
    dotenv::dotenv().ok();
    // 初始化日志
    tracing_subscriber::fmt::init();

    let conn = get_connection().await?;
    let _exec = setup::create_user_table(&conn).await?;

    let app = Router::new().route("/", get(login)).layer(Extension(conn));
    let addr = SocketAddr::from(([0, 0, 0, 0], 8555));
    tracing::debug!("listening on {}", addr);

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`

    Ok(axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|_| AppError::Serve)?)
}

async fn get_connection() -> Result<DatabaseConnection, AppError> {
    let db_url =
        env::var("DATABASE_URL").map_err(|_| AppError::MissingEnvParam("DATABASE_URL".into()))?;
    let conn = Database::connect(db_url)
        .await
        .map_err(|_| AppError::DbConnect)?;
    Ok(conn)
}
async fn login() {}
