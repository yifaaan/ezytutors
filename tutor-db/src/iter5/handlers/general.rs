use actix_web::{web, HttpResponse};

use crate::state::AppState;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut vis_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, vis_count);
    *vis_count += 1;
    HttpResponse::Ok().json(&response)
}
