use actix_web::{HttpResponse, web, get};

#[get("/")]
async fn greet() -> Option<HttpResponse> {
    Some(HttpResponse::Ok().body("Hello! from stan host"))
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  // config.route("/", web::post().to(greet));
  config.service(greet);
}
