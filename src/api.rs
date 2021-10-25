use super::structs::{
    IpifyResponse, UpdateIp, UpdateIpRespone, VerifcationResponse, ZoneListResponse,
};

use reqwest::header;

const API_ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

async fn client_builder() -> reqwest::Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        "Content-Type",
        header::HeaderValue::from_static("application/json"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .build()
        .unwrap()
}

pub async fn verify_key(apikey: String) -> Result<&'static str, String> {
    let client = client_builder().await;
    let req = client
        .get(format!("{}/user/tokens/verify", API_ENDPOINT))
        .header("Authorization", format!("Bearer {}", apikey))
        .send()
        .await
        .unwrap()
        .json::<VerifcationResponse>()
        .await
        .unwrap();

    if req.result.status == "active" {
        Ok("Key is active")
    } else {
        Err(format!(
            "Key is not working got messages: {:#?}",
            req.messages
        ))
    }
}

pub async fn update_domain(
    apikey: String,
    zoneid: &str,
    name: &str,
    domainid: &str,
    r#type: &str,
    newip: &str,
) {
    let patch = UpdateIp {
        r#type: r#type.to_owned(),
        name: name.to_owned(),
        content: newip.to_owned(),
        ttl: 1,
    };
    let client = client_builder().await;
    let req = client
        .put(format!(
            "{}/zones/{}/dns_records{}",
            API_ENDPOINT, zoneid, domainid
        ))
        .header("Authorization", format!("Bearer {}", apikey))
        .json(&patch)
        .send()
        .await;
    match req {
        Ok(a) => {
            let json = a.json::<UpdateIpRespone>().await;
            match json {
                Ok(k) => {
                    if k.success {
                        println!("updated domain: {}", k.result.name)
                    } else {
                        println!("failed to update domain: {}", k.result.name)
                    }
                }
                Err(e) => {
                    println!("failed to convert response to json error: {}", e)
                }
            }
        }
        Err(e) => {
            println!("failed to update domain {}, got {}", name, e)
        }
    }
}

// todo write domain ip update logic
pub async fn update_zone(apikey: &str, zoneid: String) {
    let client = client_builder().await;
    let ipreq = client
        .get("https://api64.ipify.org?format=json")
        .send()
        .await
        .expect("could not get ip from ipify")
        .json::<IpifyResponse>()
        .await
        .expect("could convert ipify respone to json");
    let ip = ipreq.ip;
    let req = client
        .get(format!("{}/zones/{}/dns_records", API_ENDPOINT, zoneid))
        .header("Authorization", format!("Bearer {}", apikey))
        .send()
        .await;
    match req {
        Ok(req) => {
            let json = &req.json::<ZoneListResponse>().await;
            match json {
                Ok(a) => {
                    for domain in &a.result {
                        if (domain.r#type == "A" || domain.r#type == "AAAA") && domain.content != ip {
                            update_domain(
                                apikey.to_string(),
                                &zoneid,
                                &domain.name,
                                &domain.id,
                                &domain.r#type,
                                &ip,
                            )
                            .await;
                        } 
                    }
                }
                Err(e) => {
                    println!("there way an error,the following error occured:\n{}", e)
                }
            }
        }
        Err(req) => {
            println!("there way an error, the following error:\n{}", req)
        }
    }
}
