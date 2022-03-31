use crate::db::PgPool;
use crate::hosts::{models, requests};
use crate::schema::hosts;
use diesel::{ExpressionMethods, RunQueryDsl};

use actix_web::{get, post, web, HttpResponse};

#[get("/")]
async fn get_all_hosts(pool: web::Data<PgPool>) -> Option<HttpResponse> {
  let conn = &pool.get().unwrap();
  let hosts_list = hosts::table.load::<models::Hosts>(conn).unwrap();
  Some(HttpResponse::Ok().json(hosts_list))
}

#[post("/")]
async fn insert_new_host(
  payload: web::Json<requests::HostsRequest>,
  pool: web::Data<PgPool>,
) -> HttpResponse {
  use crate::schema::hosts::dsl::*;

  // TODO: Create response struct consist of message: String
  let response_body: String;
  let host_existence = models::Hosts::check_existing(payload.name.clone(), pool.clone()).unwrap();

  match host_existence {
    true => response_body = format!("Host already exist"),
    false => {
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

      response_body = format!("Affected Rows: {}", result);
    }
  }

  HttpResponse::Ok().body(response_body)
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_hosts).service(insert_new_host);
}
