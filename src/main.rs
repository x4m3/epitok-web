mod auth;
mod root;
mod utils;

use actix_identity::{CookieIdentityPolicy, Identity, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use std::{env, io::Result, net::SocketAddr};

#[macro_use]
extern crate log;

#[actix_rt::main]
async fn main() -> Result<()> {
    env::set_var("RUST_LOG", "info");
    env_logger::init();

    info!("{} - {}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));

    let port: u16 = match env::var("PORT") {
        Ok(port_str) => port_str.parse().expect("Could not use provided port."),
        Err(_) => 4343,
    };

    let app = HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::new("[RETURNED HTTP %s] [TOOK %Dms] %r"))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&[0; 32])
                    .name("epitok-auth")
                    .secure(false),
            ))
            .service(actix_files::Files::new("/static", "static"))
            .route("/", web::get().to(crate::root::root))
            .route("/auth/signin", web::get().to(crate::root::root))
            .route("/auth/signin", web::post().to(crate::auth::sign_in))
            .route("/auth/test", web::get().to(auth_test))
    });

    info!("starting server on http://localhost:{}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    app.bind(addr)?.run().await
}

async fn auth_test(id: Identity) -> Result<String> {
    if let Some(id) = id.identity() {
        Ok(format!(
            "autologin: {} === login: {}",
            crate::utils::cookie_get_autologin(&id),
            crate::utils::cookie_get_login(&id)
        ))
    } else {
        Ok("wassup anon".to_owned())
    }
}
