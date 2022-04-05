use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddProductLimitRequest {
  pub product_name: String,
  pub build_limit: Option<String>,
  pub bandwith_limit: Option<String>,
  pub site_limit: Option<String>,
}
