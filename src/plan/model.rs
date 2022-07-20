use crate::db::PgPool;
use crate::lib::types::timeunit::TimeUnit;
use crate::plan::request::*;
use crate::schema::plans;
use crate::schema::plans::dsl::*;
use actix_web::web;
use diesel::QueryResult;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde::{Deserialize, Serialize};

#[derive(Queryable, Debug, Clone, Deserialize, Serialize)]
pub struct Plan {
    pub id: i32,
    pub hosts_id: i32,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<i32>,
    pub price_unit: Option<String>,
    pub price_timeunit: Option<TimeUnit>,
    pub price_desc: Option<String>,
    pub concurrent_build: Option<i32>,
    pub concurrent_build_unit: Option<String>,
    pub concurrent_build_timeunit: Option<TimeUnit>,
    pub concurrent_build_desc: Option<String>,
    pub bandwidth: Option<i32>,
    pub bandwidth_unit: Option<String>,
    pub bandwidth_timeunit: Option<TimeUnit>,
    pub bandwidth_desc: Option<String>,
    pub build: Option<i32>,
    pub build_unit: Option<String>,
    pub build_timeunit: Option<TimeUnit>,
    pub build_desc: Option<String>,
    pub analytic: Option<bool>,
    pub analytic_price: Option<i32>,
    pub analytic_unit: Option<String>,
    pub analytic_timeunit: Option<TimeUnit>,
    pub analytic_desc: Option<String>,
    pub plan_url: Option<String>,
    pub currency: Option<String>,
    pub discounted_price: Option<i32>,
    pub free_domain: Option<bool>,
    pub domain_extension: Option<String>,
    pub database_benefit: Option<bool>,
    pub page_data: Option<String>,
}

impl Plan {
    pub fn get_all(pool: web::Data<PgPool>) -> QueryResult<Vec<Plan>> {
        let conn = &pool.get().unwrap();
        plans::table.order(price.asc()).load::<Plan>(conn)
    }

    pub fn get_one(plan_name: String, pool: web::Data<PgPool>) -> QueryResult<Plan> {
        let conn = &pool.get().unwrap();
        plans.filter(&name.eq(plan_name)).first::<Plan>(conn)
    }

    pub fn get_all_by_host(host_id: i32, pool: web::Data<PgPool>) -> QueryResult<Vec<Plan>> {
        let conn = &pool.get().unwrap();
        plans.filter(&hosts_id.eq(host_id)).load::<Plan>(conn)
    }

    pub fn add(host_id: i32, body: AddPlanRequest, pool: web::Data<PgPool>) -> QueryResult<usize> {
        let conn = &pool.get().unwrap();
        let data = (
            &hosts_id.eq(&host_id),
            &name.eq(&body.name),
            &description.eq(&body.description),
            &price.eq(&body.price),
            &price_unit.eq(&body.price_unit),
            &price_timeunit.eq(&body.price_timeunit),
            &price_desc.eq(&body.price_desc),
            &concurrent_build.eq(&body.concurrent_build),
            &concurrent_build_unit.eq(&body.concurrent_build_unit),
            &concurrent_build_timeunit.eq(&body.concurrent_build_timeunit),
            &concurrent_build_desc.eq(&body.concurrent_build_desc),
            &bandwidth.eq(&body.bandwidth),
            &bandwidth_unit.eq(&body.bandwidth_unit),
            &bandwidth_timeunit.eq(&body.bandwidth_timeunit),
            &bandwidth_desc.eq(&body.bandwidth_desc),
            &build.eq(&body.build),
            &build_unit.eq(&body.build_unit),
            &build_timeunit.eq(&body.build_timeunit),
            &build_desc.eq(&body.build_desc),
            &analytic.eq(&body.analytic),
            &analytic_price.eq(&body.analytic_price),
            &analytic_unit.eq(&body.analytic_unit),
            &analytic_timeunit.eq(&body.analytic_timeunit),
            &analytic_desc.eq(&body.analytic_desc),
            &plan_url.eq(&body.plan_url),
        );
        diesel::insert_into(plans).values(data).execute(conn)
    }

    pub fn update(
        body: web::Json<UpdatePlanRequest>,
        pool: web::Data<PgPool>,
    ) -> QueryResult<Plan> {
        let conn = &pool.get().unwrap();

        let data = UpdatePlanRequest {
            name: body.name.clone(),
            description: body.description.clone(),
            price: body.price.clone(),
            price_unit: body.price_unit.clone(),
            price_timeunit: body.price_timeunit.clone(),
            price_desc: body.price_desc.clone(),
            concurrent_build: body.concurrent_build.clone(),
            concurrent_build_unit: body.concurrent_build_unit.clone(),
            concurrent_build_timeunit: body.concurrent_build_timeunit.clone(),
            concurrent_build_desc: body.concurrent_build_desc.clone(),
            bandwidth: body.bandwidth.clone(),
            bandwidth_unit: body.bandwidth_unit.clone(),
            bandwidth_timeunit: body.bandwidth_timeunit.clone(),
            bandwidth_desc: body.bandwidth_desc.clone(),
            build: body.build.clone(),
            build_unit: body.build_unit.clone(),
            build_timeunit: body.build_timeunit.clone(),
            build_desc: body.build_desc.clone(),
            analytic: body.analytic.clone(),
            analytic_price: body.analytic_price.clone(),
            analytic_unit: body.analytic_unit.clone(),
            analytic_timeunit: body.analytic_timeunit.clone(),
            analytic_desc: body.analytic_desc.clone(),
            plan_url: body.plan_url.clone(),
            currency: body.currency.clone(),
            discounted_price: body.discounted_price.clone(),
            free_domain: body.free_domain.clone(),
            domain_extension: body.domain_extension.clone(),
            database_benefit: body.database_benefit.clone(),
            page_data: body.page_data.clone(),
        };
        diesel::update(plans)
            .filter(name.eq(body.name.clone()))
            .set(data)
            .get_result::<Plan>(conn)
    }
}
