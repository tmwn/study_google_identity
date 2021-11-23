use study_google_auth::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let configuration = get_configuration();
    let application = Application::build(configuration).await?;
    application.run_until_stopped().await
}
