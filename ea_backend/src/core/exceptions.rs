use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use sqlx::Error as SqlxError;
use serde::Serialize;
use thiserror::Error;


#[derive(Debug, Error)]
pub enum AppError {
    // 400
    #[error("validation error: {0}")]
    Validation(String),

    // 401
    #[error("unauthorized")]
    Unauthorized,

    #[error("invalid token")]
    InvalidToken,

    #[error("token expired")]
    TokenExpired,

    // 403
    #[error("forbidden")]
    Forbidden,

    // 404
    #[error("not found")]
    NotFound,

    // 409
    #[error("conflict")]
    Conflict,

    // 503
    #[error("service unavailable")]
    Unavailable,

    // 500
    #[error("internal server error")]
    DBError,
}


#[derive(Serialize)]
struct ErrorResponse {
    message: String,
}


impl ResponseError for AppError {
    fn status_code(&self) -> StatusCode {
        match self {
            AppError::Validation(_) => StatusCode::BAD_REQUEST,
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::InvalidToken => StatusCode::UNAUTHORIZED,
            AppError::TokenExpired => StatusCode::UNAUTHORIZED,
            AppError::Forbidden => StatusCode::FORBIDDEN,
            AppError::NotFound => StatusCode::NOT_FOUND,
            AppError::Conflict => StatusCode::CONFLICT,
            AppError::Unavailable => StatusCode::SERVICE_UNAVAILABLE,
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




impl From<SqlxError> for AppError {
    fn from(err: SqlxError) -> Self {
        match err {
            SqlxError::RowNotFound => AppError::NotFound,

            SqlxError::Database(db_err) => {
                // PostgreSQL のエラーコードを見る
                if let Some(code) = db_err.code() {
                    match code.as_ref() {
                        "23505" => AppError::Conflict, // unique_violation
                        _ => AppError::DBError,
                    }
                } else {
                    AppError::DBError
                }
            }

            _ => AppError::DBError,
        }
    }
}