use crate::{
    ar::{UserActiveModel, UserEntity, UserModel},
    error::AppError,
};
use async_trait::async_trait;
use sea_orm::{ActiveModelTrait, DbConn, EntityTrait, Set};

#[async_trait]
pub trait UserService {
    async fn find_user_by_id(self, id: i32) -> Result<Option<UserModel>, AppError>;
    async fn insert_user(self, name: String, password: String) -> Result<UserModel, AppError>;
}

#[async_trait]
impl UserService for &DbConn {
    async fn find_user_by_id(self, id: i32) -> Result<Option<UserModel>, AppError> {
        Ok(UserEntity::find_by_id(id).one(self).await.map_err(|e| {
            tracing::debug!(?e);
            AppError::Find("User".into(), format!("id={}", id))
        })?)
    }

    async fn insert_user(self, name: String, password: String) -> Result<UserModel, AppError> {
        let user = UserActiveModel {
            name: Set(name.to_owned()),
            password: Set(password.to_owned()),
            ..Default::default()
        };

        Ok(user.insert(self).await.map_err(|e| {
            tracing::debug!(?e);
            AppError::Create(
                "User".into(),
                format!("name = {}, passord = {}", name, password),
            )
        })?)
    }
}
