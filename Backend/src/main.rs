use sqlx::postgres::PgPool;
use std::env;
use dotenv::dotenv;
use actix_web::{web, App, HttpServer, HttpResponse, Responder};

mod models;
mod db;
mod handlers;
 
#[actix_web::get("/greet/{name}")]
async fn greet(name: web::Path<(String)>) -> impl Responder {
    HttpResponse::Ok().body(format!("Hello, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok(); // This loads the variables from .env into the environment
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPool::connect(&database_url).await.expect("Failed to connect to Postgres");
    
    let port = 8080;
    println!("Starting server at: http://127.0.0.1/8080");

//for testing purposes simply run code and go to http://127.0.0.1:8080/user/2 to see user json file


    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(greet)
            .service(handlers::user_handler::get_user)
        })
            .bind(("127.0.0.1", port))?
            .run()
            .await
}