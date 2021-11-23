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
}

pub fn get_configuration() -> Settings {
    // Hint: use config crate to use different settings for dev and prod.
    Settings {
        application: ApplicationSettings {
            port: 8080,
            host: "127.0.0.1".into(),
        },
        auth: AuthSettings {
            admin_google_emails: AdminEmails(vec!["tmwn@tmwn.org"]),
        },
    }
}
