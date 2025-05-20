use actix_web::{web, App, HttpServer, HttpResponse, Responder};
use dotenv::dotenv;
use sqlx::mysql::MySqlPool;
use std::env;

#[derive(serde::Deserialize, serde::Serialize)]
struct User {
    id: Option<i32>,
    name: String,
    email: String,
}

async fn get_users(pool: web::Data<MySqlPool>) -> impl Responder {
    let users = sqlx::query_as!(User, "SELECT * FROM users")
        .fetch_all(pool.get_ref())
        .await
        .unwrap();
    HttpResponse::Ok().json(users)
}

async fn get_user(id: web::Path<i32>, pool: web::Data<MySqlPool>) -> impl Responder {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = ?", id)
        .fetch_one(pool.get_ref())
        .await;
    match user {
        Ok(user) => HttpResponse::Ok().json(user),
        Err(_) => HttpResponse::NotFound().finish(),
    }
}

async fn create_user(user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query!(
        "INSERT INTO users (name, email) VALUES (?, ?)",
        user.name,
        user.email
    )
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(_) => HttpResponse::Created().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn update_user(id: web::Path<i32>, user: web::Json<User>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query!(
        "UPDATE users SET name = ?, email = ? WHERE id = ?",
        user.name,
        user.email,
        id
    )
    .execute(pool.get_ref())
    .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn delete_user(id: web::Path<i32>, pool: web::Data<MySqlPool>) -> impl Responder {
    let result = sqlx::query!("DELETE FROM users WHERE id = ?", id)
        .execute(pool.get_ref())
        .await;
    match result {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    env_logger::init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL not set");
    let pool = MySqlPool::connect(&database_url).await.unwrap();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .route("/users", web::get().to(get_users))
            .route("/users/{id}", web::get().to(get_user))
            .route("/users", web::post().to(create_user))
            .route("/users/{id}", web::put().to(update_user))
            .route("/users/{id}", web::delete().to(delete_user))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
