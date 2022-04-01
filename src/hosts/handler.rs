use crate::db::PgPool;
use crate::hosts::{models, requests};
use crate::schema::hosts;
use crate::schema::hosts::dsl::*;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_hosts(pool: web::Data<PgPool>) -> HttpResponse {
  let conn = &pool.get().unwrap();
  let hosts_list = hosts::table.load::<models::Hosts>(conn).unwrap();
  HttpResponse::Ok().json(hosts_list)
}

#[get("/{host_name}")]
async fn get_host(path: web::Path<String>, pool: web::Data<PgPool>) -> HttpResponse {
  let conn = &pool.get().unwrap();
  let host_name = path.into_inner();
  let existing_host = hosts
    .filter(&name.eq(host_name))
    .first::<models::Hosts>(conn);
  match existing_host {
    Ok(host) => HttpResponse::Ok().json(host),
    Err(_) => HttpResponse::Ok().body("Data not found"),
  }
}

#[post("/")]
async fn insert_new_host(
  payload: web::Json<requests::HostsRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  // TODO: Create response struct consist of message: String
  let conn = &pool.get().unwrap();

  let data = (
    &name.eq(&payload.name),
    &description.eq(&payload.description),
    &url.eq(&payload.url),
  );
  let result = diesel::insert_into(hosts)
    .values(data)
    .execute(conn)
    .unwrap();

  HttpResponse::Ok().body(format!("Affected Rows: {}", result))
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config
    .service(get_all_hosts)
    .service(get_host)
    .service(insert_new_host);
}
