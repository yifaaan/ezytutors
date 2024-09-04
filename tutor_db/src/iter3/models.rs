use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

impl From<web::Json<Course>> for Course {
    fn from(value: web::Json<Course>) -> Self {
        Self {
            course_id: value.course_id,
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            posted_time: value.posted_time,
        }
    }
}
