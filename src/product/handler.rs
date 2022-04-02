use crate::db::PgPool;
use crate::host::model::Host;
use crate::product::model;
use crate::product::request;

use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_product(pool: web::Data<PgPool>) -> HttpResponse {
  let product_list = model::Product::get_all(pool);
  match product_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

#[post("/")]
async fn insert_new_product(
  req: web::Json<request::ProductRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let host_id = Host::get_id(req.host_name.clone(), pool.clone());
  match host_id {
    Ok(id) => HttpResponse::Ok().body(format!("The id is {:?}:", id)),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_product).service(insert_new_product);
}
