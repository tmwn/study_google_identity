use jsonwebtoken::EncodingKey;
use study_google_auth::{configuration::get_configuration, startup::Application};

pub struct TestApp {
    pub address: String,
    pub encoding_key: EncodingKey,
}

const TEST_SECRET: &str = "12345";

pub async fn spawn_app() -> TestApp {
    std::env::set_var("SECRET", TEST_SECRET);
    let configuration = {
        let mut c = get_configuration();
        // Use a random OS port.
        c.application.port = 0;
        c
    };
    let encoding_key = configuration.auth.encoding_key.clone();

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let test_app = TestApp {
        address: format!("http://localhost:{}", application.port()),
        encoding_key,
    };
    test_app
}
