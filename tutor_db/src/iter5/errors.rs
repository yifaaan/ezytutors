use std::fmt;

use actix_web::{error, http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
}

impl EzyTutorError {
    fn error_response(&self) -> String {
        match self {
            Self::DBError(msg) => {
                println!("Database error occurred: {:?}", msg);
                "Database error".into()
            }
            Self::ActixError(msg) => {
                println!("Server error occurred: {:?}", msg);
                "Internal server error".into()
            }
            Self::NotFound(msg) => {
                println!("Not found error occurred: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(value: actix_web::error::Error) -> Self {
        Self::ActixError(value.to_string())
    }
}

impl From<sqlx::error::Error> for EzyTutorError {
    fn from(value: sqlx::error::Error) -> Self {
        Self::DBError(value.to_string())
    }
}

/// 实现该trait 可将自定义的错误类型转换成HttpResponse
impl error::ResponseError for EzyTutorError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::DBError(msg) | Self::ActixError(msg) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(msg) => StatusCode::NOT_FOUND,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        // body
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}
