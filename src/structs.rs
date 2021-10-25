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

#[derive(Debug, Serialize)]
pub struct UpdateIp {
    pub r#type: String,
    pub name: String,
    pub content: String,
    pub ttl: u16,
    pub proxied: bool,
}
