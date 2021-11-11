use super::api::Updated;
use lettre::transport::smtp::authentication::Credentials;
use lettre::transport::smtp::response::Response;
use lettre::transport::smtp::Error;
use lettre::{Message, SmtpTransport, Transport};

pub async fn sendmail(
    username: String,
    password: String,
    smtp: String,
    domains: Vec<Updated>,
) -> Result<Response, Error> {
    let mut message = "The following domains have been updated:\n".to_string();
    let mut count = 1;
    for domain in domains {
        if domain.status {
            message.push_str(&format!("{}. {}\n", count, domain.domain));
        }
        count += 1;
    }
    let email = Message::builder()
        .from(username.parse().unwrap())
        .to(username.parse().unwrap())
        .subject("Domains updated")
        .body(message)
        .unwrap();

    let creds = Credentials::new(username, password);

    let mailer = SmtpTransport::relay(&smtp)
        .unwrap()
        .credentials(creds)
        .build();

    mailer.send(&email)
}
