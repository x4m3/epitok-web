use actix_identity::Identity;
use actix_web::{HttpResponse, Responder};
use askama::Template;
use epitok::event::{list_events, Event};

pub async fn root(id: Identity) -> impl Responder {
    match id.identity() {
        Some(id) => home_page(id).await,
        None => sign_in_page(),
    }
}

#[derive(Template)]
#[template(path = "sign_in.html")]
struct SignInTemplate {}

fn sign_in_page() -> HttpResponse {
    let content = SignInTemplate {};
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}

#[derive(Template)]
#[template(path = "homepage.html")]
struct HomePageTemplate<'a> {
    login: &'a str,
    events: Vec<Event>,
    date: &'a str,
}

async fn home_page(id: String) -> HttpResponse {
    // let date = chrono::Local::today();
    let date = chrono::NaiveDate::parse_from_str("2020-06-30", "%Y-%m-%d").unwrap();
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

    let content = HomePageTemplate {
        login: crate::cookie::get_login(&id),
        events,
        date: &formatted_date,
    };
    match content.render() {
        Ok(content) => HttpResponse::Ok().content_type("text/html").body(content),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Could not render template: <code>{}</code>", e)),
    }
}
