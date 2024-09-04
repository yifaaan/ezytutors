use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::{course_routes, general_routes};
use sqlx::PgPool;
use state::AppState;
use std::{env, io, sync::Mutex};
#[path = "../iter3/db_access.rs"]
mod db_access;
#[path = "../iter3/handlers.rs"]
mod handlers;
#[path = "../iter3/models.rs"]
mod models;
#[path = "../iter3/routes.rs"]
mod routes;
#[path = "../iter3/state.rs"]
mod state;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set in .env file");
    let db_poop = PgPool::connect(&database_url).await.unwrap();

    let shared_data = web::Data::new(AppState {
        health_check_response: "I'm good.".to_string(),
        visit_count: Mutex::new(0),
        db: db_poop,
    });

    let app = move || {
        App::new()
            .app_data(shared_data.clone())
            .configure(general_routes)
            .configure(course_routes)
    };
    HttpServer::new(app).bind("127.0.0.1:3000")?.run().await
}
