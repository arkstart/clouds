use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, sql_query};
use serde::{Deserialize, Serialize};

use crate::db::PgPool;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Hosts {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}
