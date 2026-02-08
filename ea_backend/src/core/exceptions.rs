use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    #[error("not found")]
    NotFound,
    #[error("DB error")]
    DBError,
    #[error("unauthorized")]
    Unauthorized
}


#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}


impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::DBError => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let body = ErrorResponse {
            message: self.to_string(),
        };

        HttpResponse::build(self.status_code()).json(body)
        
    }
}
