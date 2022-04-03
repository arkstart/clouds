use crate::db::PgPool;
use crate::host::model::Host;
use crate::lib::auth::Auth;
use crate::lib::error::{ErrResponse, ErrType};
use crate::product::{model, request};
use actix_web::{get, post, web, HttpRequest, HttpResponse};

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
  let response = {
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
  };

  Auth::validate_and_response(req, response)
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_product).service(insert_new_product);
}
