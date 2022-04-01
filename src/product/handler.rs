use crate::db::PgPool;
use crate::host::{model, request};

use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_product(pool: web::Data<PgPool>) -> HttpResponse {
  let host_list = model::Host::get_all(pool);
  match host_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_product);
}
