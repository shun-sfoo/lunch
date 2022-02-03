use backend::Builder;
use sea_orm::prelude::*;

#[allow(dead_code)]
#[derive(Builder)]
#[builder(name = "payload")]
struct Model {
    pub id: i32,
    pub appid: String,
    pub name: String,
    pub is_delta: String,
    pub metadata_signature: String,
    pub metadata_size: i64,
    pub sha256_hex: String,
    pub size: i64,
    pub target_version: String,
    pub version: i32,
    pub published: bool,
    pub publish_at: DateTime,
}

fn main() {}
