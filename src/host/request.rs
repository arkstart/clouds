use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostRequest {
  pub name: String,
  pub always_free: Option<bool>,
  pub free_tier: Option<bool>,
  pub frontend_support: Option<bool>,
  pub backend_support: Option<bool>,
  pub database_support: Option<bool>,
  pub description: Option<String>,
  pub url: Option<String>,
}
