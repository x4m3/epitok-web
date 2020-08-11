mod auth;
mod cookie;
mod root;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::{middleware, web, App, HttpServer};
use rand::Rng;
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

    // Generate a random 32 byte key for cookies
    let cookies_private_key = rand::thread_rng().gen::<[u8; 32]>();

    let app = HttpServer::new(move || {
        App::new()
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookies_private_key)
                    .name("epitok-auth")
                    .secure(false),
            ))
            .wrap(middleware::Logger::new("[RETURNED HTTP %s] [TOOK %Dms] %r"))
            .service(actix_files::Files::new("/static", "static"))
            .route("/", web::get().to(crate::root::root))
            .route("/auth/signin", web::get().to(crate::root::root))
            .route("/auth/signin", web::post().to(crate::auth::sign_in))
            .route("/auth/signout", web::get().to(crate::auth::sign_out))
    });

    info!("starting server on http://localhost:{}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    app.bind(addr)?.run().await
}
