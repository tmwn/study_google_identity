use study_google_auth::{configuration::get_configuration, startup::Application};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("[Study Google Identity] Starting main");
    let configuration = get_configuration();
    let application = Application::build(configuration).await?;
    println!("Starting app on :{}", application.port());
    application.run_until_stopped().await
}
