use std::collections::HashMap;

use axum::{extract::Extension, Json};
use reqwest::StatusCode;
use sea_orm::DbConn;
use tracing::info;

use crate::{
    model::{Claims, User},
    service::{ClaimService, UserService},
};

pub async fn me(claims: Claims, Extension(ref conn): Extension<DbConn>) -> Json<Option<User>> {
    let id = claims.id;
    match conn.find_user_by_id(id).await.unwrap() {
        Some(user) => Json(Some(User {
            id: user.id,
            username: user.name,
            token: None,
        })),
        _ => Json(None),
    }
}

pub async fn register(
    Json(data): Json<HashMap<String, String>>,
    Extension(ref conn): Extension<DbConn>,
) -> (StatusCode, Json<Option<User>>) {
    //todo genarate token
    info!(?data);
    let name = data.get("username").unwrap();
    let password = data.get("password").unwrap();
    if let Ok(user) = conn.insert_user(name.into(), password.into()).await {
        let claims = Claims::new(user.id, user.name);
        let token = claims.generate();
        let user = User {
            id: user.id,
            username: name.into(),
            token: Some(token),
        };

        return (StatusCode::OK, Json(Some(user)));
    }

    return (StatusCode::BAD_REQUEST, Json(None));
}
