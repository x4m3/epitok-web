use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

pub async fn root(id: Identity) -> impl Responder {
    let id = match id.identity() {
        Some(id) => id,
        None => {
            let content = RootTemplate {};
            return match content.render() {
                Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
                Err(e) => HttpResponse::InternalServerError()
                    .content_type("text/html")
                    .body(format!("Could not render template: <code>{}</code>", e)),
            };
        }
    };
    HttpResponse::Ok().body(format!(
        "autologin {}\nlogin {}",
        crate::cookie::get_autologin(&id),
        crate::cookie::get_login(&id)
    ))
}
