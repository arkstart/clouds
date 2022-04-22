use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddPlanRequest {
  pub host_name: String,
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
