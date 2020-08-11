use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

pub async fn root(id: Identity) -> impl Responder {
    match id.identity() {
        Some(id) => home_page(id),
        None => sign_in_page(),
    }
}

fn sign_in_page() -> HttpResponse {
    let content = RootTemplate {};
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}

fn home_page(id: String) -> HttpResponse {
    HttpResponse::Ok().body(format!(
        "autologin {}\nlogin {}",
        crate::cookie::get_autologin(&id),
        crate::cookie::get_login(&id)
    ))
}
