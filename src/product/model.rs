use crate::db::PgPool;
use crate::plan::request::*;
use crate::schema::plans;
use crate::schema::plans::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};

use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub hosts_id: i32,
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub product_url: Option<String>,
    pub free_tier: Option<bool>,
    pub free_trial: Option<bool>,
    pub base_price: Option<f32>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
    pub price_desc: Option<String>,
    pub multi_pricing: Option<bool>,
}
