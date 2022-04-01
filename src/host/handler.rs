use crate::db::PgPool;
use crate::host::{model, request};

use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_host(pool: web::Data<PgPool>) -> HttpResponse {
  let host_list = model::Host::get_all(pool);
  match host_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

#[get("/{host_name}")]
async fn get_host(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
  let host_name = path.into_inner();
  match model::Host::get_one(host_name, pool) {
    Ok(host) => HttpResponse::Ok().json(host),
    Err(_) => HttpResponse::Ok().body("Data not found"),
  }
}

#[post("/")]
async fn insert_new_host(
  req: web::Json<request::HostRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  // TODO: Create response struct consist of message: String

  match model::Host::add(req, pool) {
    Ok(res) => HttpResponse::Ok().body(format!("Affected Rows: {}", res)),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_host)
    .service(get_host)
    .service(insert_new_host);
}
