mod api;
mod mailer;
mod responses;
mod structs;
use clap::{Command, Arg};

use std::fs::File;

#[tokio::main]
async fn main() {
    let matches = Command::new("Cloudflareupdated")
        .version("0.1.0")
        .author("Luna D Dragon")
        .about("Update your public ip for cloudflare automatically")
        .arg(
            Arg::new("config")
                .short('c')
                .long("config")
                .value_name("SETTINGS_FILE")
                .help("Path to config file if it is not in the current directory")
                .takes_value(true),
        )
        .arg(
            Arg::new("verify")
                .long("verify")
                .help("check if api key works")
                .takes_value(false),
        )
        .get_matches();
    let config = matches.value_of("config").unwrap_or("./settings.yaml");
    let f = File::open(config).expect("Could not open settings file.");
    let settings: structs::Settings = serde_yaml::from_reader(f).expect("Could not read values.");
    if matches.is_present("verify") {
        let verify = api::verify_key(settings.apikey).await;
        match verify {
            Ok(a) => println!("{}", a),
            Err(e) => println!("{}", e),
        }
    } else {
        for zone in settings.zones {
            let update = api::update_zone(&settings.apikey, zone.id, zone.domains).await;
            match update {
                Some(updated_domains) => match &settings.stmp_creds {
                    Some(creds) => {
                        if !updated_domains.is_empty() {
                            // the clones are a hack to circumvent issues with borrowing
                            match mailer::sendmail(
                                creds.username.clone(),
                                creds.password.clone(),
                                creds.stmpserver.clone(),
                                updated_domains,
                            )
                            .await
                            {
                                Ok(_) => return,
                                Err(e) => panic!("{}", e),
                            }
                        }
                    }
                    None => return,
                },
                None => return,
            }
        }
    }
}
