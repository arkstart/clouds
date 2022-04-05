use crate::db::PgPool;
use crate::host::model::Host;
use crate::lib::error::{ErrResponse, ErrType};
use crate::product::{model, request, product_limit};
use actix_web::{get, http::header, post, web, HttpRequest, HttpResponse};
use std::env;

#[get("/")]
async fn get_all_product(pool: web::Data<PgPool>) -> HttpResponse {
  let product_list = model::Product::get_all(pool);
  match product_list {
    Ok(list) => HttpResponse::Ok().json(list),
    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
  }
}

#[post("/")]
async fn insert_new_product(
  req: HttpRequest,
  body: web::Json<request::AddProductRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let req_auth = req.headers().get(header::AUTHORIZATION);
  let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN must be set");

  match req_auth {
    Some(token) => {
      if token.to_str().unwrap() == auth_token {
        let host_id = Host::get_id(body.host_name.clone(), pool.clone());

        if let Ok(id) = host_id {
          let result = model::Product::add(id, body.clone(), pool);
          match result {
            Ok(res) => HttpResponse::Ok().body(format!("Affected Row(s): {}", res)),
            Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
          }
        } else {
          ErrResponse::new_message(ErrType::BadRequest, "host_name not found".to_string())
        }
      } else {
        ErrResponse::new_message(ErrType::Unauthorized, "Invalid Authorization".to_string())
      }
    }
    None => ErrResponse::new_message(ErrType::Unauthorized, "Authorization not set".to_string()),
  }
}

#[post("/limit")]
async fn insert_new_product_limit(
  req: HttpRequest,
  body: web::Json<product_limit::request::AddProductLimitRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  let req_auth = req.headers().get(header::AUTHORIZATION);
  let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN must be set");

  match req_auth {
    Some(token) => {
      if token.to_str().unwrap() == auth_token {
        let product_id = model::Product::get_id(body.product_name.clone(), pool.clone());

        if let Ok(id) = product_id {
          let result = product_limit::model::ProductLimit::add(id, body.clone(), pool);
          match result {
            Ok(res) => HttpResponse::Ok().body(format!("Affected Row(s): {}", res)),
            Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
          }
        } else {
          ErrResponse::new_message(ErrType::BadRequest, "product_name not found".to_string())
        }
      } else {
        ErrResponse::new_message(ErrType::Unauthorized, "Invalid Authorization".to_string())
      }
    }
    None => ErrResponse::new_message(ErrType::Unauthorized, "Authorization not set".to_string()),
  }
}

/// Routing for product
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_product)
    .service(insert_new_product)
    .service(insert_new_product_limit);
}
