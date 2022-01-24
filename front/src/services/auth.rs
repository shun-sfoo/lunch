use crate::{
    error::Error,
    types::{LoginInfoWrapper, UserInfoWrapper},
};

use super::request_post;

pub async fn login(login_info: LoginInfoWrapper) -> Result<UserInfoWrapper, Error> {
    request_post::<LoginInfoWrapper, UserInfoWrapper>("user/login".to_string(), login_info).await
}
