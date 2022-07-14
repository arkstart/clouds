use crate::schema::hosts;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "hosts"]
pub struct HostRequest {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<String>,
    pub always_free: Option<bool>,
    pub free_tier: Option<bool>,
    pub frontend_support: Option<bool>,
    pub backend_support: Option<bool>,
    pub database_support: Option<bool>,
    pub template: Option<String>,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct HostFilterRequestParam {
    pub always_free: Option<bool>,
    pub free_tier: Option<bool>,
    pub frontend_support: Option<bool>,
    pub backend_support: Option<bool>,
    pub database_support: Option<bool>,
}
