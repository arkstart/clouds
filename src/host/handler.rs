use crate::db::PgPool;
use crate::host::{model, request};
use crate::lib::auth::Auth;
use crate::lib::error::{ErrResponse, ErrType};
use actix_web::{get, post, web, HttpRequest, HttpResponse};

#[get("/")]
async fn get_all_host(pool: web::Data<PgPool>) -> HttpResponse {
  let host_list = model::Host::get_all(pool);
  match host_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  }
}

#[get("/{host_name}")]
async fn get_host(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
  let host_name = path.into_inner();
  match model::Host::get_one(host_name, pool) {
    Ok(host) => HttpResponse::Ok().json(host),
    Err(_) => ErrResponse::new_message(ErrType::BadRequest, "Host name not found".to_string()),
  }
}

#[post("/")]
async fn insert_new_host(
  req: HttpRequest,
  body: web::Json<request::HostRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let response = match model::Host::add(body, pool) {
    Ok(res) => HttpResponse::Ok().body(format!("Affected Rows: {}", res)),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  };

  Auth::validate_and_response(req, response)
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_host)
    .service(get_host)
    .service(insert_new_host);
}
