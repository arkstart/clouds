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
  pub build_limit: Option<String>,
  pub bandwidth_limit: Option<String>,
  pub site_limit: Option<String>,
  pub https_support: Option<bool>,
  pub free_domain: Option<bool>,
  pub custom_domain: Option<bool>,
  pub domain_extension: Option<String>,
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
      &build_limit.eq(&body.build_limit),
      &bandwidth_limit.eq(&body.bandwidth_limit),
      &site_limit.eq(&body.site_limit),
      &https_support.eq(&body.https_support),
      &free_domain.eq(&body.free_domain),
      &custom_domain.eq(&body.custom_domain),
      &domain_extension.eq(&body.domain_extension)
    );
    diesel::insert_into(products).values(data).execute(conn)
  }
}
