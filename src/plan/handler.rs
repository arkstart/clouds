use crate::db::PgPool;
use crate::host::model::Host;
use crate::lib::error::{ErrResponse, ErrType};
use crate::plan::{model, request};
use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_plan(pool: web::Data<PgPool>) -> HttpResponse {
  let plan_list = model::Plan::get_all(pool);
  match plan_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  }
}

#[get("/{plan_name}")]
async fn get_plan(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
  let plan_name = path.into_inner();
  match model::Plan::get_one(plan_name, pool) {
    Ok(plan) => HttpResponse::Ok().json(plan),
    Err(_) => ErrResponse::new_message(ErrType::BadRequest, "Plan name not found".to_string()),
  }
}

#[get("/hosts/{host_name}")]
async fn get_host_plan(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
  let host_name = path.into_inner();
  if let Ok(id) = Host::get_id(host_name, pool.clone()) {
    match model::Plan::get_all_by_host(id, pool) {
      Ok(res) => HttpResponse::Ok().json(res),
      Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
    }
  } else {
    ErrResponse::new_message(ErrType::BadRequest, "Host name not found".to_string())
  }
}

#[post("/")]
async fn insert_new_plan(
  body: web::Json<request::AddPlanRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let host_id = Host::get_id(body.host_name.clone(), pool.clone());

  if let Ok(id) = host_id {
    let result = model::Plan::add(id, body.clone(), pool);
    match result {
      Ok(res) => HttpResponse::Ok().body(format!("Affected Row(s): {}", res)),
      Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
    }
  } else {
    ErrResponse::new_message(ErrType::BadRequest, "host_name not found".to_string())
  }
}

/// Routing for product
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_plan)
    .service(get_plan)
    .service(get_host_plan)
    .service(insert_new_plan);
}
