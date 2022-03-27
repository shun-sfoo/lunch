use std::collections::HashMap;

use axum::{extract::Extension, Json};
use sea_orm::DbConn;
use tracing::info;

use crate::{
    model::{Claims, User, UserResponse},
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
) -> Result<Json<UserResponse>, &'static str> {
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

        return Ok(Json(UserResponse { user }));
    }

    return Err("err");
}

pub async fn fake_users() -> Json<Vec<User>> {
    let mut users = Vec::new();
    let user1 = User {
        id: 1,
        username: "123".into(),
        token: None,
    };

    let user2 = User {
        id: 2,
        username: "124".into(),
        token: None,
    };

    users.push(user1);
    users.push(user2);

    Json(users)
}
