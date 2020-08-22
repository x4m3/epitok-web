use actix_identity::Identity;
use actix_web::Responder;

pub async fn root(id: Identity) -> impl Responder {
    match id.identity() {
        Some(id) => crate::list_events::render_events(id, None).await,
        None => crate::signin::render_page(),
    }
}
