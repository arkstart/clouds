use crate::db::PgPool;
use crate::host::request::HostRequest;
use crate::schema::hosts;
use crate::schema::hosts::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Host {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}

impl Host {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Host>> {
    let conn = &pool.get().unwrap();
    hosts::table.load::<Host>(conn)
  }

  pub fn get_one(host_name: String, pool: web::Data<PgPool>) -> QueryResult<Host> {
    let conn = &pool.get().unwrap();
    hosts.filter(&name.eq(host_name)).first::<Host>(conn)
  }

  pub fn get_id(host_name: String, pool: web::Data<PgPool>) -> QueryResult<i32> {
    let conn = &pool.get().unwrap();
    hosts.filter(&name.eq(host_name)).select(id).first::<i32>(conn)
  }

  pub fn add(body: web::Json<HostRequest>, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();

    let data = (
      &name.eq(&body.name),
      &description.eq(&body.description),
      &url.eq(&body.url),
    );
    diesel::insert_into(hosts).values(data).execute(conn)
  }
}
