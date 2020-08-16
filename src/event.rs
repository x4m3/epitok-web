use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder};
use askama::Template;
use epitok::event::{get_event, Event};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Params {
    year: String,
    module: String,
    instance: String,
    acti: String,
    event: String,
}

pub async fn event(params: web::Path<Params>, id: Identity) -> impl Responder {
    match id.identity() {
        Some(id) => event_page(params, id).await,
        None => HttpResponse::Found()
            .header(http::header::LOCATION, "/")
            .finish(),
    }
}

#[derive(Template)]
#[template(path = "event.html")]
struct EventTemplate<'a> {
    login: &'a str,
    event: Event,
}

async fn event_page(params: web::Path<Params>, id: String) -> HttpResponse {
    let mut event = match get_event(
        crate::cookie::get_autologin(&id),
        &params.year,
        &params.module,
        &params.instance,
        &params.acti,
        &params.event,
    )
    .await
    {
        Ok(event) => event,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("could not get event: {}", e));
        }
    };

    if let Err(e) = event
        .fetch_students(crate::cookie::get_autologin(&id))
        .await
    {
        return HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("failed to get list of students: {}", e));
    }

    let content = EventTemplate {
        login: crate::cookie::get_login(&id),
        event,
    };
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}
