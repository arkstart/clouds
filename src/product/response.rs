use serde::{Deserialize, Serialize};
use crate::product::product_limit;

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ProductWithSpecResponse {
  pub id: i32,
  pub hosts_id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
  pub product_limit: product_limit::model::ProductLimit
}
