mod auth;

use std::collections::HashMap;

pub use auth::{
    LoginInfo, LoginInfoWrapper, RegisterInfo, RegisterInfoWrapper, UserInfo, UserInfoWrapper,
};
use serde::{Deserialize, Serialize};

/// Conduit api error info for Unprocessable Entity error
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct ErrorInfo {
    pub errors: HashMap<String, Vec<String>>,
}

pub type DeleteWrapper = HashMap<(), ()>;
