use actix_web::web;
use serde::{Deserialize, Serialize};

///  data captured in the tutor registration form
#[derive(Serialize, Deserialize, Debug)]
pub struct TutorRegisterForm {
    pub username: String,
    pub password: String,
    pub confirmation: String,
    pub name: String,
    pub imageurl: String,
    pub profile: String,
}

/// store the response from the tutor web service
#[derive(Serialize, Deserialize, Debug)]
pub struct TutorResponse {
    pub tutor_id: i32,
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

#[derive(Serialize, Deserialize, Debug, sqlx::FromRow)]
pub struct User {
    pub username: String,
    pub tutor_id: Option<i32>,
    pub user_password: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TutorSigninForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct NewCourse {
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_duration: String,
    pub course_structure: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

/// 从tutor web server数据库服务中发来的数据
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct NewCourseResponse {
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: String,
    pub course_format: String,
    pub course_structure: Option<String>,
    pub course_duration: String,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    pub posted_time: String,
}

impl From<web::Json<NewCourseResponse>> for NewCourseResponse {
    fn from(new_course: web::Json<NewCourseResponse>) -> Self {
        NewCourseResponse {
            tutor_id: new_course.tutor_id,
            course_id: new_course.course_id,
            course_name: new_course.course_name.clone(),
            course_description: new_course.course_description.clone(),
            course_format: new_course.course_format.clone(),
            course_structure: new_course.course_structure.clone(),
            course_duration: new_course.course_duration.clone(),
            course_price: new_course.course_price,
            course_language: new_course.course_language.clone(),
            course_level: new_course.course_level.clone(),
            posted_time: new_course.posted_time.clone(),
        }
    }
}
