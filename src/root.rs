use actix_web::{HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

pub async fn root() -> impl Responder {
    let content = RootTemplate {};

    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!(
                "<code>Internal server error: could not render template: {}</code>",
                e
            )),
    }
}
