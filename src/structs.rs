use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub id: String,
    pub domains: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Smtp {
    pub username: String,
    pub password: String,
    pub stmpserver: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Settings {
    pub apikey: String,
    pub zones: Vec<Zone>,
    pub stmp_creds: Option<Smtp>,
}

#[derive(Debug, Serialize)]
pub struct UpdateIp {
    pub r#type: String,
    pub name: String,
    pub content: String,
    pub ttl: u16,
    pub proxied: bool,
}
