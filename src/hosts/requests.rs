use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostRequest {
  pub name: String,
  pub description: String,
  pub url: String,
}
