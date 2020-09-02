use actix_identity::Identity;
use actix_web::{HttpRequest, HttpResponse, Responder};
use askama::Template;
use epitok::event::{list_events, Event};

#[derive(Template)]
#[template(path = "homepage.html")]
struct ListEventsTemplate<'a> {
    login: &'a str,
    events: Vec<Event>,
    date: &'a str,
    yesterday: &'a str,
    tomorrow: &'a str,
}

pub async fn specific_date(id: Identity, req: HttpRequest) -> impl Responder {
    match id.identity() {
        Some(id) => match req.match_info().get("date") {
            Some(date) => render_events(id, Some(date)).await,
            None => HttpResponse::BadRequest().body("invalid date requested"),
        },
        None => crate::signin::render_page(),
    }
}

pub async fn render_events(id: String, date: Option<&str>) -> HttpResponse {
    let date = match date {
        Some(date) => match chrono::NaiveDate::parse_from_str(date, "%Y-%m-%d") {
            Ok(date) => date,
            Err(_) => return HttpResponse::BadRequest().body("invalid date requested"),
        },
        None => chrono::Local::today().naive_local(),
    };
    let date_yyyymmdd = date.format("%Y-%m-%d").to_string();

    let mut events = Vec::new();
    if let Err(e) = list_events(
        &mut events,
        crate::cookie::get_autologin(&id),
        &date_yyyymmdd,
    )
    .await
    {
        return HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("could not get list of events from intra: {}", e));
    }

    let formatted_date = date.format("%A, %B %d").to_string();
    let yesterday = (date - chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();
    let tomorrow = (date + chrono::Duration::days(1))
        .format("%Y-%m-%d")
        .to_string();

    let content = ListEventsTemplate {
        login: crate::cookie::get_login(&id),
        events,
        date: &formatted_date,
        yesterday: &yesterday,
        tomorrow: &tomorrow,
    };
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}
