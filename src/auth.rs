use actix_web::{web, HttpResponse, Responder};
use epitok::auth::Auth;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct FormData {
    autologin: String,
}

pub async fn sign_in(form: web::Form<FormData>) -> impl Responder {
    let mut auth = Auth::new();
    match auth.sign_in(&form.autologin).await {
        Ok(()) => (),
        Err(e) => {
            return HttpResponse::Forbidden()
                .content_type("text/html")
                .body(e.to_string())
        }
    };
    let autologin = match auth.autologin() {
        Some(autologin) => autologin,
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("oops wtf")
        }
    };
    let login = match auth.login() {
        Some(login) => login,
        None => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body("oops wtf")
        }
    };
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("autologin {}, login {}", autologin, login))
}
