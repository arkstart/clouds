use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddProductRequest {
  pub host_name: String,
  pub name: String,
  pub description: String,
  pub url: Option<String>,
  pub free: Option<bool>,
  pub pricing: Option<String>,
}
