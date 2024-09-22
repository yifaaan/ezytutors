use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Course {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub posted_time: Option<NaiveDateTime>,
}

// Convert http json data to Rust data
impl From<web::Json<Course>> for Course {
    fn from(value: web::Json<Course>) -> Self {
        Course {
            course_id: value.course_id,
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            posted_time: value.posted_time,
        }
    }
}
