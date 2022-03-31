use crate::db::PgPool;
use crate::diesel::RunQueryDsl;
use crate::hosts::models;
use crate::schema::hosts;

use actix_web::{get, web, HttpResponse};

#[get("/")]
async fn get_all_hosts(pool: web::Data<PgPool>) -> Option<HttpResponse> {
  let conn = &pool.get().unwrap();
  let hosts_list = hosts::table.load::<models::Hosts>(conn).unwrap();
  Some(HttpResponse::Ok().json(hosts_list))
}

/// Routing for hosts
pub fn route(config: &mut web::ServiceConfig) {
  config.service(get_all_hosts);
}
