mod root;

use actix_web::{middleware, App, HttpServer};
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
            .wrap(middleware::Logger::new("[HTTP %s] [TOOK %Dms] [URL %U]"))
            .service(actix_files::Files::new("/static", "static"))
            .service(crate::root::root)
    });

    info!("starting server on http://localhost:{}", port);

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    app.bind(addr)?.run().await
}
