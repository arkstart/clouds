use crate::db::PgPool;
use crate::host::model::Host;
use crate::lib::error::{ErrResponse, ErrType};
use crate::product::{model, request};
use actix_web::{get, post, put, web, HttpResponse};

#[get("/")]
async fn get_all_product(pool: web::Data<PgPool>) -> HttpResponse {
    let product_list = model::Product::get_all(pool);
    match product_list {
        Ok(list) => HttpResponse::Ok().json(list),
        Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
    }
}

#[get("/{product_name}")]
async fn get_product(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
    let product_name = path.into_inner();
    match model::Product::get_one(product_name, pool) {
        Ok(plan) => HttpResponse::Ok().json(plan),
        Err(_) => {
            ErrResponse::new_message(ErrType::BadRequest, "Product name not found".to_string())
        }
    }
}

#[post("/")]
async fn insert_new_product(
    body: web::Json<request::AddProductRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    let host_id = Host::get_id(body.host_name.clone(), pool.clone());

    if let Ok(id) = host_id {
        let product_category = model::Product::check_category(&body.category);

        match product_category {
            true => {
                let result = model::Product::add(id, body.clone(), pool);
                match result {
                    Ok(res) => HttpResponse::Ok().body(format!("Affected Row(s): {}", res)),
                    Err(e) => ErrResponse::new(ErrType::InternalServerError, e.to_string()),
                }
            }
            false => {
                let error_message =
                    "Category should be either ANLT, STOR, DTBS, CMPT, or CTNR".to_string();
                ErrResponse::new_message(ErrType::BadRequest, error_message)
            }
        }
    } else {
        ErrResponse::new_message(ErrType::BadRequest, "host_name not found".to_string())
    }
}

#[put("/")]
async fn update_product(
    body: web::Json<request::UpdateProductRequest>,
    pool: web::Data<PgPool>,
) -> HttpResponse {
    if let Some(category) = body.category.clone() {
        let product_category = model::Product::check_category(&category);

        match product_category {
            true => match model::Product::update(body, pool) {
                Ok(res) => HttpResponse::Ok().json(res),
                Err(e) => ErrResponse::new(ErrType::BadRequest, e.to_string()),
            },
            false => {
                let error_message =
                    String::from("Category should be either ANLT, STOR, DTBS, CMPT, or CTNR");
                ErrResponse::new_message(ErrType::BadRequest, error_message)
            }
        }
    } else {
        match model::Product::update(body, pool) {
            Ok(res) => HttpResponse::Ok().json(res),
            Err(e) => ErrResponse::new(ErrType::BadRequest, e.to_string()),
        }
    }
}

/// Routing for product
pub fn route(config: &mut web::ServiceConfig) {
    config
        .service(get_all_product)
        .service(get_product)
        .service(insert_new_product)
        .service(update_product);
}
