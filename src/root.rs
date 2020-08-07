use actix_web::{get, HttpResponse, Responder};
use askama::Template;

#[derive(Template)]
#[template(path = "root.html")]
struct RootTemplate {}

#[get("/")]
pub async fn root() -> impl Responder {
    let content = RootTemplate {}.render().unwrap();
    HttpResponse::Ok().content_type("text/html").body(content)
}
