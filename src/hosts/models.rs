use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Hosts {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}
