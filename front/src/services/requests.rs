use gloo::storage::{LocalStorage, Storage};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use serde::{de::DeserializeOwned, Serialize};

use crate::error::Error;

const TOKEN_KEY: &str = "yew.token";

lazy_static! {
    /// JWT token read from local storage
    pub static ref TOKEN: RwLock<Option<String>> = {
        if let Ok(token) = LocalStorage::get(TOKEN_KEY) {
            RwLock::new(Some(token))
        }else {
            RwLock::new(None)
        }
    };
}

pub fn set_token(token: Option<String>) {
    if let Some(t) = token.clone() {
        LocalStorage::set(TOKEN_KEY, t);
    } else {
        LocalStorage::delete(TOKEN_KEY);
    }

    let mut token_lock = TOKEN.write();
    *token_lock = token;
}

pub async fn request_post<B, T>(url: String, body: B) -> Result<T, Error>
where
    T: DeserializeOwned + 'static + std::fmt::Debug,
    B: Serialize + std::fmt::Debug,
{
    todo!()
}
