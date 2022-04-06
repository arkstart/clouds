use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddProductRequest {
  pub host_name: String,
  pub name: String,
  pub description: String,
  pub url: String,
  pub build_limit: Option<String>,
  pub bandwith_limit: Option<String>,
  pub site_limit: Option<String>,
  pub https_support: Option<bool>,
  pub free_domain: Option<bool>,
  pub custom_domain: Option<bool>,
  pub domain_extension: Option<bool>,
}
