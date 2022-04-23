use crate::db::PgPool;
use crate::host::{model, request};
use crate::schema::hosts;
use crate::schema::hosts::dsl::*;

use actix_web::web;
use diesel::{ExpressionMethods, QueryDsl, QueryResult, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Host {
  pub id: i32,
  pub name: String,
  pub always_free: Option<bool>,
  pub free_tier: Option<bool>,
  pub frontend_support: Option<bool>,
  pub backend_support: Option<bool>,
  pub database_support: Option<bool>,
  pub description: Option<String>,
  pub url: Option<String>,
  pub product_based: Option<bool>,
  pub plan_based: Option<bool>
}

impl Host {
  // Set order by descending after we made the products endpoint
  pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Host>> {
    let conn = &pool.get().unwrap();
    hosts::table.order(name.desc()).load::<Host>(conn)
  }

  pub fn get_all_name(pool: web::Data<PgPool>) -> QueryResult<Vec<String>> {
    let conn = &pool.get().unwrap();
    hosts::table.select(hosts::name).order(name.desc()).load::<String>(conn)
  }

  pub fn get_one(host_name: String, pool: web::Data<PgPool>) -> QueryResult<Host> {
    let conn = &pool.get().unwrap();
    hosts.filter(&name.eq(host_name)).first::<Host>(conn)
  }

  pub fn get_id(host_name: String, pool: web::Data<PgPool>) -> QueryResult<i32> {
    let conn = &pool.get().unwrap();
    hosts
      .filter(&name.eq(host_name))
      .select(id)
      .first::<i32>(conn)
  }

  pub fn filter(
    param: web::Query<request::HostFilterRequestParam>,
    pool: web::Data<PgPool>,
  ) -> QueryResult<Vec<model::Host>> {
    let mut query = hosts.filter(always_free.is_not_null()).into_boxed();

    if let Some(free) = param.always_free {
      query = query.filter(always_free.eq(free));
    }

    if let Some(free) = param.free_tier {
      query = query.filter(free_tier.eq(free));
    }

    if let Some(support) = param.frontend_support {
      query = query.filter(frontend_support.eq(support))
    }
    if let Some(support) = param.backend_support {
      query = query.filter(backend_support.eq(support))
    }

    if let Some(support) = param.database_support {
      query = query.filter(database_support.eq(support))
    }

    let conn = &pool.get().unwrap();
    query.get_results::<model::Host>(conn)
  }

  pub fn add(body: web::Json<request::HostRequest>, pool: web::Data<PgPool>) -> QueryResult<usize> {
    let conn = &pool.get().unwrap();

    let data = (
      &name.eq(&body.name),
      &description.eq(&body.description),
      &url.eq(&body.url),
      &always_free.eq(&body.always_free),
      &free_tier.eq(&body.free_tier),
      &frontend_support.eq(&body.frontend_support),
      &backend_support.eq(&body.backend_support),
      &database_support.eq(&body.database_support),
      &product_based.eq(&body.product_based),
      &plan_based.eq(&body.plan_based)
    );
    diesel::insert_into(hosts).values(data).execute(conn)
  }

  pub fn update(
    body: web::Json<request::HostRequest>,
    pool: web::Data<PgPool>,
  ) -> QueryResult<model::Host> {
    let conn = &pool.get().unwrap();

    let data = request::HostRequest {
      name: body.name.clone(),
      description: body.description.clone(),
      url: body.url.clone(),
      always_free: body.always_free.clone(),
      free_tier: body.free_tier.clone(),
      frontend_support: body.frontend_support.clone(),
      backend_support: body.backend_support.clone(),
      database_support: body.database_support.clone(),
      product_based: body.product_based.clone(),
      plan_based: body.plan_based.clone()
    };
    diesel::update(hosts)
      .filter(name.eq(body.name.clone()))
      .set(data)
      .get_result::<model::Host>(conn)
  }
}
