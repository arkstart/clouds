use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use std::env;

mod hosts;

async fn serve_web(address: String) -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new().service(web::scope("/hosts").configure(hosts::handler::route))
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

    serve_web(address).await
}
