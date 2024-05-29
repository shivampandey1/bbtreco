use actix_web::{web, HttpResponse, Responder};
use sqlx::PgPool;
use crate::models::user::User;
use crate::db::db_functions;

#[actix_web::post("/user")]
pub async fn create_user(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match db_functions::create_user(&pool, &user).await {
        Ok(_) => HttpResponse::Ok().body("User created successfully"),
        Err(e) => {
            eprintln!("Failed to create user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to create user")
        }
    }
}

#[actix_web::get("/user/{user_id}")]
pub async fn get_user(pool: web::Data<PgPool>, user_id: web::Path<i32>) -> impl Responder {
    //test that we got here
    match db_functions::read_user(&pool, *user_id).await {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(e) => {
            eprintln!("Failed to get user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to get user")
        }
    }
}

//#[actix_web::put("/user")]
pub async fn update_user(pool: web::Data<PgPool>, user: web::Json<User>) -> impl Responder {
    match db_functions::update_user(&pool, &user).await {
        Ok(_) => HttpResponse::Ok().body("User updated successfully"),
        Err(e) => {
            eprintln!("Failed to update user: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to update user")
        }
    }
}