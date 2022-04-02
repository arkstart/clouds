use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct ProductRequest {
  pub host_name: String,
  pub name: String,
  pub description: String,
  pub url: Option<String>,
  pub free: Option<bool>,
  pub pricing: Option<String>,
  pub build_limit: Option<String>,
  pub bandwith_limit: Option<String>,
  pub site_limit: Option<String>,
}
