use crate::errors::EzyTutorError;
use actix_web::web;
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;

#[derive(Deserialize, Serialize, Debug, Clone, sqlx::FromRow)]
pub struct Course {
    // 数据库生成
    pub course_id: i32,
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
    // 数据库生成
    pub posted_time: Option<NaiveDateTime>,
}

/// 客户端请求添加课程
#[derive(Deserialize, Debug, Clone)]
pub struct CreateCourse {
    pub tutor_id: i32,
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<CreateCourse>> for CreateCourse {
    fn from(value: web::Json<CreateCourse>) -> Self {
        CreateCourse {
            tutor_id: value.tutor_id,
            course_name: value.course_name.clone(),
            course_description: value.course_description.clone(),
            course_duration: value.course_duration.clone(),
            course_format: value.course_format.clone(),
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
            course_price: value.course_price,
            course_structure: value.course_structure.clone(),
        }
    }
}

// impl TryFrom<web::Json<CreateCourse>> for CreateCourse {
//     type Error = EzyTutorError;
//     fn try_from(value: web::Json<CreateCourse>) -> Result<Self, Self::Error> {
//         Ok(CreateCourse {
//             tutor_id: value.tutor_id,
//             course_name: value.course_name.clone(),
//             course_description: value.course_description.clone(),
//             course_duration: value.course_duration.clone(),
//             course_format: value.course_format.clone(),
//             course_language: value.course_language.clone(),
//             course_level: value.course_level.clone(),
//             course_price: value.course_price,
//             course_structure: value.course_structure.clone(),
//         })
//     }
// }

/// 客户请求更新课程
#[derive(Deserialize, Debug, Clone)]
pub struct UpdateCourse {
    pub course_name: String,
    pub course_description: Option<String>,
    pub course_format: Option<String>,
    pub course_structure: Option<String>,
    pub course_duration: Option<String>,
    pub course_price: Option<i32>,
    pub course_language: Option<String>,
    pub course_level: Option<String>,
}

impl From<web::Json<UpdateCourse>> for UpdateCourse {
    fn from(value: web::Json<UpdateCourse>) -> Self {
        Self {
            course_name: value.course_name.clone(),
            course_description: value.course_description.clone(),
            course_duration: value.course_duration.clone(),
            course_format: value.course_format.clone(),
            course_language: value.course_language.clone(),
            course_level: value.course_level.clone(),
            course_price: value.course_price,
            course_structure: value.course_structure.clone(),
        }
    }
}
