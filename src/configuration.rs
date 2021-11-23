pub struct Settings {
    pub application: ApplicationSettings,
}

pub struct ApplicationSettings {
    pub port: u16,
    pub host: String,
}

pub fn get_configuration() -> Settings {
    // Hint: use config crate to use different settings for dev and prod.
    Settings {
        application: ApplicationSettings {
            port: 8080,
            host: "127.0.0.1".into(),
        },
    }
}
