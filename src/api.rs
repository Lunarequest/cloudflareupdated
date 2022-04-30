use super::responses::{IpifyResponse, UpdateIpRespone, VerifcationResponse, ZoneListResponse};
use super::structs::UpdateIp;

use reqwest::header;

const API_ENDPOINT: &str = "https://api.cloudflare.com/client/v4";

#[derive(Debug)]
pub struct Updated {
	pub domain: String,
	pub status: bool,
}

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
		Err("Key is not working, please check if it has expired".to_string())
	}
}

pub async fn update_domain(
	apikey: String,
	zoneid: &str,
	name: &str,
	domainid: &str,
	r#type: &str,
	newip: &str,
) -> bool {
	let patch = UpdateIp {
		r#type: r#type.to_owned(),
		name: name.to_owned(),
		content: newip.to_owned(),
		ttl: 1,
		proxied: true,
	};
	let client = client_builder().await;
	let req = client
		.put(format!(
			"{}/zones/{}/dns_records/{}",
			API_ENDPOINT, zoneid, domainid
		))
		.header("Authorization", format!("Bearer {}", apikey))
		.json(&patch)
		.send()
		.await;
	match req {
		Ok(a) => {
			let json = &a.json::<UpdateIpRespone>().await;
			match json {
				Ok(k) => {
					if k.success {
						println!("updated domain: {}", k.result.name);
						true
					} else {
						println!("failed to update domain: {}", k.result.name);
						false
					}
				}
				Err(e) => {
					println!("failed to convert response to json error: {}", e);
					false
				}
			}
		}
		Err(e) => {
			println!("failed to update domain {}, got {}", name, e);
			false
		}
	}
}

pub async fn update_zone(
	apikey: &str,
	zoneid: String,
	accepteddomain: Vec<String>,
) -> Option<Vec<Updated>> {
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
	let mut updated_domains: Vec<Updated> = vec![];
	match req {
		Ok(req) => {
			let json = &req.json::<ZoneListResponse>().await;
			match json {
				Ok(a) => {
					for domain in &a.result {
						if (domain.r#type == "A" || domain.r#type == "AAAA")
							&& domain.content != ip && accepteddomain
							.contains(&domain.name.to_string())
						{
							let name = &domain.name;
							let updated = update_domain(
								apikey.to_string(),
								&zoneid,
								name,
								&domain.id,
								&domain.r#type,
								&ip,
							)
							.await;
							let entery = Updated {
								domain: name.to_owned(),
								status: updated,
							};
							updated_domains.push(entery)
						}
					}
					Some(updated_domains)
				}
				Err(e) => {
					println!("there way an error,the following error occured:\n{}", e);
					None
				}
			}
		}
		Err(req) => {
			println!("there way an error, the following error:\n{}", req);
			None
		}
	}
}
