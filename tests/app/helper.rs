use study_google_auth::{configuration::get_configuration, startup::Application};

pub struct TestApp {
    pub address: String,
}

pub async fn spawn_app() -> TestApp {
    std::env::set_var("SECRET", "12345");
    let configuration = {
        let mut c = get_configuration();
        // Use a random OS port.
        c.application.port = 0;
        c
    };

    let application = Application::build(configuration)
        .await
        .expect("Failed to build application");

    let test_app = TestApp {
        address: format!("http://localhost:{}", application.port()),
    };
    test_app
}
