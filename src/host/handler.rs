use crate::db::PgPool;
use crate::host::{model, request};
use crate::lib::error::{ErrResponse, ErrType};
use actix_web::{get, http::header, post, web, HttpRequest, HttpResponse};
use std::env;

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
  let req_auth = req.headers().get(header::AUTHORIZATION);
  let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN must be set");

  match req_auth {
    Some(token) => {
      if token.to_str().unwrap() == auth_token {
        match model::Host::add(body, pool) {
          Ok(res) => HttpResponse::Ok().body(format!("Affected Rows: {}", res)),
          Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
        }
      } else {
        ErrResponse::new_message(ErrType::Unauthorized, "Invalid Authorization".to_string())
      }
    }
    None => ErrResponse::new_message(ErrType::Unauthorized, "Authorization not set".to_string()),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_host)
    .service(get_host)
    .service(insert_new_host);
}
