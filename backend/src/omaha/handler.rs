use axum::{body::Bytes, extract::Extension};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

use crate::entity::payload::Entity as Payload;
use crate::omaha::model::ClientRequest;
use tracing::info;

pub async fn health_check() -> &'static str {
    "server is running..."
}

pub async fn update(bytes: Bytes, Extension(ref conn): Extension<DatabaseConnection>) -> String {
    let data = String::from_utf8(bytes.to_vec()).expect("update but get request data failed");

    let client_request: ClientRequest =
        quick_xml::de::from_str(&data).expect("update but deserialize request failed");

    info!("client rquest : {:?}", client_request);
    let client_version = &client_request.app.version;

    todo!()
}

async fn find_latest_published_payload_by_appid(
    Extension(ref conn): Extension<DatabaseConnection>,
    find_app_id: &str,
) -> Option<Payload> {
    let payload = Payload::find()
        .filter(crate::entity::payload::Column::Appid.contains(find_app_id))
        .all(conn)
        .await;

    todo!()
}
