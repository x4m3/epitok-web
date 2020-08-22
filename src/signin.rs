use actix_web::HttpResponse;
use askama::Template;

#[derive(Template)]
#[template(path = "sign_in.html")]
struct SignInTemplate {}

pub fn render_page() -> HttpResponse {
    let content = SignInTemplate {};
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}
