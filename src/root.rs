use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use askama::Template;
use epitok::event::{list_events, list_events_today};

#[derive(Template)]
#[template(path = "sign_in.html")]
struct SignInTemplate {}

pub async fn root(id: Identity) -> impl Responder {
    match id.identity() {
        Some(id) => home_page(id).await,
        None => sign_in_page(),
    }
}

fn sign_in_page() -> HttpResponse {
    let content = SignInTemplate {};
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}

async fn home_page(id: String) -> HttpResponse {
    let mut events = Vec::new();
    // if let Err(e) = list_events_today(&mut events, crate::cookie::get_autologin(&id)).await {
    if let Err(e) = list_events(&mut events, crate::cookie::get_autologin(&id), "2020-06-15").await
    {
        return HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("could not get list of events from intra: {}", e));
    }

    for event in events {
        println!(
            "code: {}\ntitle: {}\nmodule: {}\nstart: {}\nend: {}\n",
            event.code(),
            event.title(),
            event.module(),
            event.start(),
            event.end(),
        );
    }

    HttpResponse::Ok().body(format!(
        "autologin {}\nlogin {}",
        crate::cookie::get_autologin(&id),
        crate::cookie::get_login(&id)
    ))
}
