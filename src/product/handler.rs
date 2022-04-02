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
  req: web::Json<request::AddProductRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let host_id = Host::get_id(req.host_name.clone(), pool.clone());

  if let Ok(id) = host_id {
    let result = model::Product::add(id, req.clone(), pool);
    match result {
      Ok(res) => HttpResponse::Ok().body(format!("Affected Row(s): {}", res)),
      Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
    }
  } else {
    HttpResponse::Ok().body(format!("host_name not found!"))
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_product).service(insert_new_product);
}
