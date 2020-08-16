mod auth;
mod cookie;
mod event;
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
            // cookie authentication
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(&cookies_private_key)
                    .name("epitok-auth")
                    .secure(false),
            ))
            // log requests
            .wrap(middleware::Logger::new("[RETURNED HTTP %s] [TOOK %Dms] %r"))
            // serve static files
            .service(actix_files::Files::new("/static", "static"))
            // root page
            .route("/", web::get().to(crate::root::root))
            // authentication
            .service(
                web::scope("/auth")
                    .route("/signin", web::post().to(crate::auth::sign_in))
                    .route("/signout", web::get().to(crate::auth::sign_out)),
            )
            // individual events
            .service(web::scope("/event").route(
                "/{year}/{module}/{instance}/{acti}/{event}",
                web::get().to(crate::event::event),
            ))

        // TODO: 404 page
        // TODO: protections?
    });

    info!("starting server on http://localhost:{}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    app.bind(addr)?.run().await
}
