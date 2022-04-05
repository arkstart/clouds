use crate::db::PgPool;
use crate::product::product_limit::request;
use crate::schema::products_limit::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ProductLimit {
  pub id: i32,
  pub products_id: i32,
  pub build_limit: Option<String>,
  pub bandwith_limit: Option<String>,
  pub site_limit: Option<String>,
}

impl ProductLimit {
  pub fn add(
    product_id: i32,
    body: request::AddProductLimitRequest,
    pool: web::Data<PgPool>,
  ) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();
    let data = (
      &products_id.eq(&product_id),
      &build_limit.eq(&body.build_limit),
      &bandwith_limit.eq(&body.bandwith_limit),
      &site_limit.eq(&body.site_limit),
    );
    diesel::insert_into(products_limit)
      .values(data)
      .execute(conn)
  }
}
