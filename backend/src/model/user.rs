use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct User {
    id: i32,
    username: String,
    token: String,
}
