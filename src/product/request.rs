use crate::schema::products;
use diesel::AsChangeset;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct AddProductRequest {
    pub host_name: String,
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub category: String,
    pub product_url: Option<String>,
    pub free_tier: Option<bool>,
    pub free_trial: Option<bool>,
    pub base_price: Option<f64>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
    pub price_desc: Option<String>,
    pub multi_pricing: Option<bool>,
}

#[derive(Queryable, Debug, Clone, Deserialize, Serialize, AsChangeset)]
#[table_name = "products"]
pub struct UpdateProductRequest {
    pub title: String,
    pub subtitle: Option<String>,
    pub description: Option<String>,
    pub category: Option<String>,
    pub product_url: Option<String>,
    pub free_tier: Option<bool>,
    pub free_trial: Option<bool>,
    pub base_price: Option<f64>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
    pub price_desc: Option<String>,
    pub multi_pricing: Option<bool>,
}


#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ProductFilterParam {
    pub category: Option<String>,
    pub free_tier: Option<bool>,
    pub free_trial: Option<bool>,
    pub base_price: Option<f64>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<String>,
}