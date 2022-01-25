use serde::{Deserialize, Serialize};

// https://github.com/google/omaha/blob/master/doc/ServerProtocolV3.md

#[derive(Debug, Deserialize)]
pub struct ClientRequest {
    #[serde(rename(deserialize = "requestid"))]
    request_id: String,
    #[serde(rename(deserialize = "sessionid"))]
    session_id: String,
    protocol: String,
    updater: String,
    updaterversion: String,
    installsource: String,
    ismachine: u8, // "1" if the client is known to be installed with system-level or administrator privileges. "0" otherwise. Default: "0".
    os: OS,
    pub app: App,
}

#[derive(Debug, Deserialize)]
struct OS {
    version: String,
    platform: String,
    sp: String,
    market_segment: String,
}

#[derive(Debug, Deserialize)]
pub struct App {
    pub appid: String,
    pub version: String,
    pub track: String,
    pub board: String,
    pub hardware_class: String,
    pub delta_okay: Option<String>,
    pub installdate: Option<i32>, // The first communication to the server should use a special value of "-1". A value of "-2" indicates that this value is not known. Default: "-2".
    pub updatecheck: Option<()>,
    pub ping: Option<Ping>,
    pub event: Option<Event>,
    pub rollback_allowed: Option<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Ping {
    active: String, //"1" if the app was active since the previous request that contained a <ping>. Otherwise, "0". If a or ad is explicitly transmitted, active may be omitted. Default: "0".
    a: String,
    r: String,
}

#[derive(Debug, Deserialize, Default)]
pub struct Event {
    pub eventtype: u32,  // 0 ~103
    pub eventresult: u8, // 0 ~10
    pub errorcode: Option<i32>,
    pub previousversion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppData {
    pub appid: String,
    // #[serde(skip_deserializing)]
    pub name: String,
    #[serde(rename(deserialize = "sha256_hex"))]
    pub sha256: String,
    pub is_delta: bool,
    pub size: i64,
    pub metadata_signature: Option<String>,
    pub metadata_size: i64,
    pub target_version: String,
    pub version: i32,
    #[serde(skip_deserializing)]
    pub sha256_hex: String,
    // public_key: String,
    // unless: String,
}
