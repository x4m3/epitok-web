use actix_identity::Identity;
use actix_web::{web, HttpResponse, Responder};
use askama::Template;
use epitok::auth::Auth;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    autologin: String,
}

#[derive(Template)]
#[template(path = "auth-failed.html")]
struct AuthFailedTemplate {
    reason: String,
}

pub async fn sign_in(form: web::Form<FormData>, id: Identity) -> impl Responder {
    let mut auth = Auth::new();

    // Try to sign in, if it fails, render the auth failed page
    if let Err(e) = auth.sign_in(&form.autologin).await {
        let content = AuthFailedTemplate {
            reason: e.to_string(),
        };
        // Try to render template
        return match content.render() {
            Ok(content) => HttpResponse::Forbidden()
                .content_type("text/html")
                .body(content),
            Err(e) => HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("Could not render template: <code>{}</code>", e)),
        };
    };

    if !crate::cookie::set(id, auth) {
        return HttpResponse::InternalServerError()
            .content_type("text/html")
            .body("failed to complete sign in (cookies)");
    }

    HttpResponse::Found().header("location", "/").finish()
}

pub async fn sign_out(id: Identity) -> impl Responder {
    id.forget();
    HttpResponse::Found().header("location", "/").finish()
}
