use crate::db::PgPool;
use crate::schema::products;
use crate::product::request::*;
use crate::schema::products::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, RunQueryDsl, QueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
  pub id: i32,
  pub hosts_id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}

impl Product {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Product>> {
    let conn = &pool.get().unwrap();
    products::table.load::<Product>(conn)
  }

  pub fn get_id(product_name: String, pool: web::Data<PgPool>) -> QueryResult<i32> {
    let conn = &pool.get().unwrap();
    products.filter(&name.eq(product_name)).select(id).first::<i32>(conn)
  }

  pub fn get_one(product_name: String, pool: web::Data<PgPool>) -> QueryResult<Product> {
    let conn = &pool.get().unwrap();
    products.filter(&name.eq(product_name)).first::<Product>(conn)
  }

  pub fn add(host_id: i32, body:AddProductRequest, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();
    let data = (
      &hosts_id.eq(&host_id),
      &name.eq(&body.name),
      &description.eq(&body.description),
      &url.eq(&body.url),
    );
    diesel::insert_into(products).values(data).execute(conn)
  }
}
