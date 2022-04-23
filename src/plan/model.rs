use crate::db::PgPool;
use crate::plan::request::AddPlanRequest;
use crate::schema::plans;
use crate::schema::plans::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
  pub id: i32,
  pub hosts_id: i32,
  pub name: Option<String>,
  pub description: Option<String>,
  pub price: Option<i32>,
  pub price_unit: Option<String>,
  pub price_timeunit: Option<String>,
  pub price_desc: Option<String>,
  // Concurrent Build
  pub concurrent_build: Option<i32>,
  pub concurrent_build_unit: Option<String>,
  pub concurrent_build_timeunit: Option<String>,
  pub concurrent_build_desc: Option<String>,
  // Bandwidth
  pub bandwidth: Option<i32>,
  pub bandwidth_unit: Option<String>,
  pub bandwidth_timeunit: Option<String>,
  pub bandwidth_desc: Option<String>,
  // Build
  pub build: Option<i32>,
  pub build_unit: Option<String>,
  pub build_timeunit: Option<String>,
  pub build_desc: Option<String>,
  // Analytic
  pub analytic: Option<bool>,
  pub analytic_price: Option<i32>,
  pub analytic_unit: Option<String>,
  pub analytic_timeunit: Option<String>,
  pub analytic_desc: Option<String>,
}

impl Plan {
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Plan>> {
    let conn = &pool.get().unwrap();
    plans::table.load::<Plan>(conn)
  }

  pub fn get_one(plan_name: String, pool: web::Data<PgPool>) -> QueryResult<Plan> {
    let conn = &pool.get().unwrap();
    plans.filter(&name.eq(plan_name)).first::<Plan>(conn)
  }

  pub fn get_all_by_host(host_id: i32, pool: web::Data<PgPool>) -> QueryResult<Vec<Plan>> {
    let conn = &pool.get().unwrap();
    plans.filter(&hosts_id.eq(host_id)).load::<Plan>(conn)
  }

  pub fn add(host_id: i32, body: AddPlanRequest, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();
    let data = (
      &hosts_id.eq(&host_id),
      &name.eq(&body.name),
      &description.eq(&body.description),
      &price.eq(&body.price),
      &price_unit.eq(&body.price_unit),
      &price_timeunit.eq(&body.price_timeunit),
      &price_desc.eq(&body.price_desc),
      &concurrent_build.eq(&body.concurrent_build),
      &concurrent_build_unit.eq(&body.concurrent_build_unit),
      &concurrent_build_timeunit.eq(&body.concurrent_build_timeunit),
      &concurrent_build_desc.eq(&body.concurrent_build_desc),
      &bandwidth.eq(&body.bandwidth),
      &bandwidth_unit.eq(&body.bandwidth_unit),
      &bandwidth_timeunit.eq(&body.bandwidth_timeunit),
      &bandwidth_desc.eq(&body.bandwidth_desc),
      &build.eq(&body.build),
      &build_unit.eq(&body.build_unit),
      &build_timeunit.eq(&body.build_timeunit),
      &build_desc.eq(&body.build_desc),
      &analytic.eq(&body.analytic),
      &analytic_price.eq(&body.analytic_price),
      &analytic_unit.eq(&body.analytic_unit),
      &analytic_timeunit.eq(&body.analytic_timeunit),
      &analytic_desc.eq(&body.analytic_desc),
    );
    diesel::insert_into(plans).values(data).execute(conn)
  }
}
