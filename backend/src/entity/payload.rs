use sea_orm::entity::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Serialize, Deserialize)]
#[sea_orm(table_name = "payloads")]
pub struct Model {
    #[sea_orm(primary_key)]
    #[serde(skip_deserializing)]
    pub id: i32,
    pub appid: String,
    pub name: String,
    pub is_delta: String,
    pub metadata_signature: Option<String>,
    pub metadata_size: i64,
    pub sha256_hex: String,
    pub size: i64,
    pub target_version: String,
    pub version: i32,
    pub published: bool,
    pub publish_at: DateTime,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        panic!("No RelationDef")
    }
}

impl ActiveModelBehavior for ActiveModel {}
