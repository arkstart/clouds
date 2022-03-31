use serde::Serialize;

#[derive(Queryable, Debug, Clone, Serialize)]
pub struct Hosts {
  pub id: i32,
  pub name: String,
  pub description: String,
  pub url: String,
}
