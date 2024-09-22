use std::fmt;

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use serde::Serialize;

#[derive(Debug, Serialize)]
pub enum EzyTutorError {
    DBError(String),
    ActixError(String),
    NotFound(String),
    InvalidInput(String),
}

#[derive(Debug, Serialize)]
pub struct MyErrorResponse {
    error_message: String,
}

impl EzyTutorError {
    /// convert a custom error struct to a user-friendly text message
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
            EzyTutorError::InvalidInput(msg) => {
                println!("Invalid parameters received: {:?}", msg);
                msg.into()
            }
        }
    }
}

impl ResponseError for EzyTutorError {
    fn status_code(&self) -> actix_web::http::StatusCode {
        match self {
            Self::DBError(_msg) | Self::ActixError(_msg) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NotFound(_msg) => StatusCode::NOT_FOUND,
            Self::InvalidInput(_msg) => StatusCode::BAD_REQUEST,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        HttpResponse::build(self.status_code()).json(MyErrorResponse {
            error_message: self.error_response(),
        })
    }
}

impl fmt::Display for EzyTutorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl From<actix_web::error::Error> for EzyTutorError {
    fn from(value: actix_web::error::Error) -> Self {
        EzyTutorError::ActixError(value.to_string())
    }
}

impl From<sqlx::error::Error> for EzyTutorError {
    fn from(value: sqlx::error::Error) -> Self {
        EzyTutorError::DBError(value.to_string())
    }
}
