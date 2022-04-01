use crate::db::PgPool;
use crate::schema::products;
// use crate::schema::products::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
  pub id: i32,
  pub hosts_id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
  pub free: bool,
  pub pricing: String,
}

impl Product {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Product>> {
    let conn = &pool.get().unwrap();
    products::table.load::<Product>(conn)
  }
}
