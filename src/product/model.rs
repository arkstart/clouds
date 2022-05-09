use crate::db::PgPool;
use crate::product::request::*;
use crate::schema::products;
use crate::schema::products::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Product {
    pub id: i32,
    pub hosts_id: i32,
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

impl Product {
    pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Product>> {
        let conn = &pool.get().unwrap();
        products::table.load::<Product>(conn)
    }

    pub fn get_one(product_name: String, pool: web::Data<PgPool>) -> QueryResult<Product> {
        let conn = &pool.get().unwrap();
        products
            .filter(&title.eq(product_name))
            .first::<Product>(conn)
    }

    pub fn add(
        host_id: i32,
        body: AddProductRequest,
        pool: web::Data<PgPool>,
    ) -> QueryResult<usize> {
        let conn = &pool.get().unwrap();
        let data = (
            &hosts_id.eq(&host_id),
            &title.eq(&body.title),
            &subtitle.eq(&body.subtitle),
            &description.eq(&body.description),
            &category.eq(&body.category),
            &product_url.eq(&body.product_url),
            &free_tier.eq(&body.free_tier),
            &free_trial.eq(&body.free_trial),
            &base_price.eq(&body.base_price),
            &price_unit.eq(&body.price_unit),
            &price_timeunit.eq(&body.price_timeunit),
            &price_desc.eq(&body.price_desc),
            &multi_pricing.eq(&body.multi_pricing),
        );
        diesel::insert_into(products).values(data).execute(conn)
    }

    pub fn update(
        body: web::Json<UpdateProductRequest>,
        pool: web::Data<PgPool>,
    ) -> QueryResult<Product> {
        let conn = &pool.get().unwrap();
        let data = UpdateProductRequest {
            title: body.title.clone(),
            subtitle: body.subtitle.clone(),
            description: body.description.clone(),
            category: body.category.clone(),
            product_url: body.product_url.clone(),
            free_tier: body.free_tier.clone(),
            free_trial: body.free_trial.clone(),
            base_price: body.base_price.clone(),
            price_unit: body.price_unit.clone(),
            price_timeunit: body.price_timeunit.clone(),
            price_desc: body.price_desc.clone(),
            multi_pricing: body.multi_pricing.clone(),
        };
        diesel::update(products)
            .filter(title.eq(body.title.clone()))
            .set(data)
            .get_result::<Product>(conn)
    }
}
