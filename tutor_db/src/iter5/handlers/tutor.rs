use crate::{
    dbaccess::tutor::*,
    errors::EzyTutorError,
    models::tutor::{NewTutor, UpdateTutor},
    state::AppState,
};
use actix_web::{web, HttpResponse};

pub async fn get_all_tutors(app_state: web::Data<AppState>) -> Result<HttpResponse, EzyTutorError> {
    get_all_tutors_db(&app_state.db)
        .await
        .map(|tutors| HttpResponse::Ok().json(tutors))
}

pub async fn get_tutor_details(
    params: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    get_tutor_details_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn post_new_tutor(
    new_tutor: web::Json<NewTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    post_new_tutor_db(&app_state.db, new_tutor.into())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn update_tutor_details(
    params: web::Path<i32>,
    update_tutor: web::Json<UpdateTutor>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    update_tutor_details_db(&app_state.db, tutor_id, update_tutor.into())
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}

pub async fn delete_tutor(
    params: web::Path<i32>,
    app_state: web::Data<AppState>,
) -> Result<HttpResponse, EzyTutorError> {
    let tutor_id = params.into_inner();
    delete_tutor_db(&app_state.db, tutor_id)
        .await
        .map(|tutor| HttpResponse::Ok().json(tutor))
}
