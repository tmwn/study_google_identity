use std::net::TcpListener;

use actix_web::{
    dev::{Server, Service},
    web, App, HttpServer,
};

use crate::{
    configuration::{AuthSettings, Settings},
    route::{
        health_check::health_check,
        login::{check_login, login, login_endpoint},
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
        let server = run(listener, configuration)?;
        Ok(Application { port, server })
    }
    pub fn port(&self) -> u16 {
        self.port
    }
    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(listener: TcpListener, settings: Settings) -> std::io::Result<Server> {
    let auth_settings = web::Data::new(settings.auth);
    let application_settings = web::Data::new(settings.application);

    let server = HttpServer::new(move || {
        App::new()
            .app_data(auth_settings.clone())
            .app_data(application_settings.clone())
            .route("/login", web::get().to(login))
            .route("/login", web::post().to(login_endpoint))
            .service(
                web::scope("")
                    .wrap_fn(|req, srv| {
                        let result = match check_login(
                            &req,
                            req.app_data::<web::Data<AuthSettings>>().unwrap(),
                        ) {
                            Ok(claims) => {
                                req.head().extensions_mut().insert(claims);
                                Ok(())
                            }
                            Err(e) => Err(e),
                        };
                        let fut = srv.call(req);
                        async move {
                            match result {
                                Ok(_) => fut.await,
                                Err(e) => Err(e.into()),
                            }
                        }
                    })
                    .route("/secret", web::get().to(secret))
                    .route("/health_check", web::get().to(health_check)),
            )
    })
    .listen(listener)?
    .run();
    Ok(server)
}
