use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Settings {
    pub application: ApplicationSettings,
    pub auth: AuthSettings,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
    pub client_id: String,
}

pub struct AdminEmails(Vec<&'static str>);

impl AdminEmails {
    pub fn contains(&self, x: &str) -> bool {
        self.0.contains(&x)
    }
}

pub struct AuthSettings {
    pub admin_google_emails: AdminEmails,
    pub encoding_key: EncodingKey,
    pub decoding_key: DecodingKey<'static>,
}

pub fn get_configuration() -> Settings {
    let secret = std::env::var("SECRET").unwrap();
    if secret.len() < 3 {
        panic!("Secret too short");
    }
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes()).into_static();
    let client_id =
        "471589670598-v8b71548rle4hpeaj6a7dcv25p7n7o8q.apps.googleusercontent.com".to_string();

    let port = match std::env::var("PORT") {
        Err(_) => 8080,
        Ok(x) => x.parse().unwrap(),
    };
    // Hint: use config crate to use different settings for dev and prod.
    Settings {
        application: ApplicationSettings {
            port,
            host: "0.0.0.0".into(),
            client_id,
        },
        auth: AuthSettings {
            admin_google_emails: AdminEmails(vec!["tmwn@tmwn.org"]),
            encoding_key,
            decoding_key,
        },
    }
}
