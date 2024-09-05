use actix_web::web;
use serde::{Deserialize, Serialize};

/// 数据库存储结构
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tutor {
    tutor_id: i32,
    tutor_name: String,
    tutor_pic_url: String,
    tutor_profile: String,
}

/// 客户端向服务器发来的结构
#[derive(Deserialize, Debug, Clone)]
pub struct NewTutor {
    pub tutor_name: String,
    pub tutor_pic_url: String,
    pub tutor_profile: String,
}

impl From<web::Json<NewTutor>> for NewTutor {
    fn from(value: web::Json<NewTutor>) -> Self {
        Self {
            tutor_name: value.tutor_name.clone(),
            tutor_pic_url: value.tutor_pic_url.clone(),
            tutor_profile: value.tutor_profile.clone(),
        }
    }
}

#[derive(Deserialize, Debug, Clone)]
pub struct UpdateTutor {
    pub tutor_name: Option<String>,
    pub tutor_pic_url: Option<String>,
    pub tutor_profile: Option<String>,
}

impl From<web::Json<UpdateTutor>> for UpdateTutor {
    fn from(value: web::Json<UpdateTutor>) -> Self {
        Self {
            tutor_name: value.tutor_name.clone(),
            tutor_pic_url: value.tutor_pic_url.clone(),
            tutor_profile: value.tutor_profile.clone(),
        }
    }
}
