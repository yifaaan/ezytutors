use actix_web::{web, App, HttpServer};
use dotenv::dotenv;
use routes::{course_routes, general_routes, tutor_routes};
use sqlx::PgPool;
use state::AppState;
use std::{env, io, sync::Mutex};

#[path = "../iter5/dbaccess/mod.rs"]
mod dbaccess;
#[path = "../iter5/errors.rs"]
mod errors;
#[path = "../iter5/handlers/mod.rs"]
mod handlers;
#[path = "../iter5/models/mod.rs"]
mod models;
#[path = "../iter5/routes.rs"]
mod routes;
#[path = "../iter5/state.rs"]
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
            .configure(tutor_routes)
    };
    let host_port = env::var("HOST_PORT").expect("HOST:PORT address is not set in .env file");
    HttpServer::new(app).bind(&host_port)?.run().await
}
