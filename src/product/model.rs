use crate::db::PgPool;
use crate::schema::products;
use crate::product::request::AddProductRequest;
use crate::schema::products::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
  pub id: i32,
  pub hosts_id: i32,
  pub name: String,
  pub description: String,
  pub url: Option<String>,
  pub free: Option<bool>,
  pub pricing: Option<String>,
}

impl Product {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Product>> {
    let conn = &pool.get().unwrap();
    products::table.load::<Product>(conn)
  }

  pub fn add(host_id: i32, req:AddProductRequest, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();
    let data = (
      &hosts_id.eq(&host_id),
      &name.eq(&req.name),
      &description.eq(&req.description),
      &url.eq(&req.url),
      &free.eq(&req.free),
      &pricing.eq(&req.pricing),
    );
    diesel::insert_into(products).values(data).execute(conn)
  }
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ProductLimit {
  pub id: i32,
  pub products_id: i32,
  pub build_limit: Option<String>,
  pub bandwith_limit: Option<String>,
  pub site_limit: Option<String>,
}
