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

    let autologin = match auth.autologin() {
        Some(autologin) => autologin,
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("oops wtf");
        }
    };
    let login = match auth.login() {
        Some(login) => login,
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("oops wtf");
        }
    };
    let id_new = format!("{}!#{}", login, autologin);
    id.remember(id_new);

    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("autologin {}, login {}", autologin, login))

    // TODO: redirection to home page (either http redirect or callback page with html redirection)

    // TODO: sign out
}
