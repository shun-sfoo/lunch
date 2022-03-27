use std::{env, net::SocketAddr};

use axum::{
    extract::Extension,
    http::Method,
    routing::{get, post},
    Router,
};
use error::AppError;
use sea_orm::{Database, DatabaseConnection};
use tower_http::cors::{Any, CorsLayer};

use crate::handler::{fake_projects, fake_users, me, register};
mod ar;
mod error;
mod handler;
mod model;
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

    let cors = CorsLayer::new()
        .allow_methods(vec![Method::GET, Method::POST, Method::OPTIONS])
        // .allow_origin(Origin::exact("http://localhost:3000".parse().unwrap()));
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/", get(login))
        .route("/me", get(me))
        .route("/register", post(register))
        .route("/projects", get(fake_projects))
        .route("/users", get(fake_users))
        .layer(Extension(conn))
        .layer(cors);
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
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
