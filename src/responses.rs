use serde::Deserialize;


#[derive(Debug, Deserialize)]
pub struct VerifcationResponse {
    pub result: VerifcationResult,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct VerifcationResult {
    pub id: String,
    pub status: String,
}

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
}

#[derive(Debug, Deserialize)]
pub struct ZoneListResponse {
    pub result: Vec<Domain>,
    pub success: bool,
}

#[derive(Debug, Deserialize)]
pub struct IpifyResponse {
    pub ip: String,
}

#[derive(Debug, Deserialize)]
pub struct UpdateIpRespone {
    pub success: bool,
    pub result: Domain,
}
