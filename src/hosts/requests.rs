use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostsRequest {
  pub name: String,
  pub description: String,
  pub url: String,
}
