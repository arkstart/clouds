use crate::db::PgPool;
use crate::host::request::HostRequest;
use crate::schema::hosts;
use crate::schema::hosts::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
  pub free: bool,
  pub pricing: String
}
