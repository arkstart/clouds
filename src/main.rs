#[macro_use]
extern crate diesel;

use actix_web::{web, App, HttpServer};
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv::dotenv;
use std::env;

mod auth;
mod db;
mod host;
mod lib;
mod migration;
mod plan;
mod schema;

async fn serve_web(address: String, pool: db::PgPool) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .wrap(HttpAuthentication::bearer(auth::check_bearer_token))
            .app_data(web::Data::new(pool.clone()))
            .route("/api/migration", web::post().to(migration::migrate))
            .service(web::scope("/api/hosts").configure(host::handler::route))
            .service(web::scope("/api/plans").configure(plan::handler::route))
    })
    .bind(address)?
    .run()
    .await
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let host = &env::var("HOST").unwrap_or("127.0.0.1".to_string());
    let port = &env::var("PORT").unwrap_or("8080".to_string());
    let address = format!("{}:{}", host, port);

    let pool = db::connect_pool();

    serve_web(address, pool).await
}
