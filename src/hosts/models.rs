use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

use crate::db::PgPool;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Hosts {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}

impl Hosts {
  pub fn check_existing(host_name: String, pool: web::Data<PgPool>) -> Option<bool> {
    use crate::schema::hosts::dsl::*;
    let conn = &pool.get().unwrap();
    let existing_host = hosts.select(name.eq(host_name)).execute(conn).unwrap();

    return Some(existing_host != 0)
  }
}
