use jsonwebtoken::{DecodingKey, EncodingKey};

pub struct Settings {
    pub application: ApplicationSettings,
    pub auth: AuthSettings,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
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

pub fn get_configuration<'a>() -> Settings {
    let secret = std::env::var("SECRET").unwrap();
    let encoding_key = EncodingKey::from_secret(secret.as_bytes());
    let decoding_key = DecodingKey::from_secret(secret.as_bytes()).into_static();

    // Hint: use config crate to use different settings for dev and prod.
    Settings {
        application: ApplicationSettings {
            port: 8080,
            host: "127.0.0.1".into(),
        },
        auth: AuthSettings {
            admin_google_emails: AdminEmails(vec!["tmwn@tmwn.org"]),
            encoding_key: encoding_key,
            decoding_key: decoding_key,
        },
    }
}
