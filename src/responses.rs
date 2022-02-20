use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Messages {
    pub code: i32,
    pub message: String,
    pub r#type: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct VerifcationResponse {
    pub result: VerifcationResult,
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
}

#[derive(Debug, Deserialize)]
pub struct VerifcationResult {
    pub id: String,
    pub status: String,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ResultInfo {
    page: u16,
    per_page: u16,
    count: u16,
    total_count: u16,
    total_pages: u16,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct MetaInfo {
    auto_added: bool,
    managed_by_apps: Option<bool>,
    managed_by_argo_tunnel: Option<bool>,
    source: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
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
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct ZoneListResponse {
    pub result: Vec<Domain>,
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
    pub result_info: ResultInfo,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct IpifyResponse {
    pub ip: String,
}
#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct UpdateIpRespone {
    pub success: bool,
    pub errors: Vec<String>,
    pub messages: Vec<Messages>,
    pub result: Domain,
}
