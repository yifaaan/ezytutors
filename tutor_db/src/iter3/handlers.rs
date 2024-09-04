use super::db_access::*;
use super::models::Course;
use super::state::AppState;
use actix_web::{web, HttpResponse};
use std::convert::TryFrom;

pub async fn health_check_handler(app_state: web::Data<AppState>) -> HttpResponse {
    let health_check_response = &app_state.health_check_response;
    let mut visit_count = app_state.visit_count.lock().unwrap();
    let response = format!("{} {} times", health_check_response, visit_count);
    *visit_count += 1;
    HttpResponse::Ok().json(&response)
}

pub async fn get_courses_for_tutor(
    params: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    // 客户发来的查询参数
    let tutor_id = params.into_inner();
    // 查询数据库
    let courses = get_courses_for_tutor_db(&app_state.db, tutor_id).await;
    // 返回查询结果
    HttpResponse::Ok().json(courses)
}

pub async fn get_course_details(
    params: web::Path<(i32, i32)>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let (tutor_id, course_id) = params.into_inner();
    let course = get_course_details_db(&app_state.db, tutor_id, course_id).await;
    HttpResponse::Ok().json(course)
}

pub async fn post_new_course(
    new_course: web::Json<Course>,
    app_state: web::Data<AppState>,
) -> HttpResponse {
    let course = post_new_course_db(&app_state.db, new_course.into()).await;
    HttpResponse::Ok().json(course)
}
