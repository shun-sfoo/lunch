use axum::{body::Bytes, extract::Extension};
use sea_orm::DatabaseConnection;

use crate::omaha::model::ClientRequest;

pub async fn health_check() -> &'static str {
    "server is running..."
}

pub async fn update(bytes: Bytes, Extension(ref conn): Extension<DatabaseConnection>) {
    let data = String::from_utf8(bytes.to_vec()).expect("update but get request data failed");
}
