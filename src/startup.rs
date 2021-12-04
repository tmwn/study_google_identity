use std::net::TcpListener;

use actix_web::{dev::Server, web, App, HttpServer};

use crate::{
    configuration::{AuthSettings, Settings},
    route::{
        health_check::health_check,
        login::{login, login_endpoint},
        secret::secret,
    },
};

pub struct Application {
    port: u16,
    server: Server,
}

impl Application {
    pub async fn build(configuration: Settings) -> std::io::Result<Application> {
        let address = format!(
            "{}:{}",
            configuration.application.host, configuration.application.port
        );
        let listener = TcpListener::bind(address)?;
        let port = listener.local_addr().unwrap().port();
        let server = run(listener, configuration.auth)?;
        Ok(Application { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run<'a>(listener: TcpListener, auth_settings: AuthSettings) -> std::io::Result<Server> {
    let auth_settings = web::Data::new(auth_settings);
    let server = HttpServer::new(move || {
        App::new()
            // .wrap(SayHi)
            .route("/health_check", web::get().to(health_check))
            .route("/secret", web::get().to(secret))
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(login_endpoint))
            .app_data(auth_settings.clone())
    })
    .listen(listener)?
    .run();
    Ok(server)
}
