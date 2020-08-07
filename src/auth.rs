use actix_web::{web, HttpResponse, Responder};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SigninData {
    autologin: String,
}

pub async fn sign_in(form: web::Form<SigninData>) -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/html")
        .body(format!("got {}", form.autologin))
}
