use actix_identity::Identity;
use actix_web::{http, web, HttpResponse, Responder};
use askama::Template;
use epitok::student::{fetch_students, Student};
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
    students: Vec<Student>,
}

async fn event_page(params: web::Path<Params>, id: String) -> HttpResponse {
    let event_code = format!(
        "/module/{}/{}/{}/{}/{}",
        params.year, params.module, params.instance, params.acti, params.event
    );

    let mut students = Vec::new();
    if let Err(e) = fetch_students(
        &mut students,
        crate::cookie::get_autologin(&id),
        &event_code,
    )
    .await
    {
        return HttpResponse::BadRequest()
            .content_type("text/html")
            .body(format!("you made an invalid request: {}", e));
    }

    let content = EventTemplate {
        login: crate::cookie::get_login(&id),
        students,
    };
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}
