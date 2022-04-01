use crate::db::PgPool;
use crate::product::model;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use crate::schema::products;

use actix_web::{get, web, HttpResponse};

#[get("/")]
async fn get_all_product(pool: web::Data<PgPool>) -> HttpResponse {
  let product_list = model::Product::get_all(pool);
  match product_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => HttpResponse::Ok().body(format!("Error {:?}:", e)),
  }
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_product);
}
