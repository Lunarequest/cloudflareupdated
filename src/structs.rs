use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub id: String,
    pub domains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub apikey: String,
    pub zones: Vec<Zone>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifcationResult {
    pub id: String,
    pub status: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Messages {
    pub code: i32,
    pub message: String,
    pub r#type: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VerifcationResponse {
    pub result: VerifcationResult,
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MetaInfo {
    auto_added: bool,
    managed_by_apps: bool,
    managed_by_argo_tunnel: bool,
    source: String,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Domain {
    pub id: String,
    pub zone_id: String,
    pub zone_name: String,
    pub name: String,
    pub r#type: String,
    pub content: String,
    pub proxiable: bool,
    pub proxied: bool,
    pub ttl: u16,
    pub locked: bool,
    pub meta: MetaInfo,
    pub created_on: String,
    pub modified_on: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ResultInfo {
    page: u16,
    per_page: u16,
    count: u16,
    total_count: u16,
    total_pages: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ZoneListResponse {
    pub result: Vec<Domain>,
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
    pub result_info: ResultInfo,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct IpifyResponse {
    pub ip: String,
}
#[derive(Debug, Serialize)]
pub struct UpdateIp {
    pub r#type: String,
    pub name: String,
    pub content: String,
    pub ttl: u16,
}
#[derive(Debug, Deserialize)]
pub struct UpdateIpRespone {
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
    pub result: Domain,
}
