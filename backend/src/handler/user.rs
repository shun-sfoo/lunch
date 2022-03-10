use std::collections::HashMap;

use axum::{extract::Extension, Json};
use sea_orm::DbConn;
use tracing::info;

use crate::model::{Claims, User};

pub async fn me(claims: Claims, Extension(ref conn): Extension<DbConn>) {}

pub async fn register(Json(data): Json<HashMap<String, String>>) {
    info!(?data);
}
