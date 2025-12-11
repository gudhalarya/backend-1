use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Debug,Error)]
pub enum CustomErrors {
    #[error("Error when the database url is not found ")]
    DbUrlNotFound,

    #[error("Error when Could not connect to the database")]
    DbConnectionError,

    #[error("User does not exist in the database")]
    UserNotExist,

    #[error("Bad Request : {0}")]
    BadRequest(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Internal Server Error")]
    InternalError,

}

impl ResponseError for CustomErrors {
    fn error_response(&self) -> HttpResponse {
        match self {
            CustomErrors::DbUrlNotFound => HttpResponse::InternalServerError().json("Database URL not found"),
            CustomErrors::DbConnectionError => HttpResponse::InternalServerError().json("Could not connect to DB"),
            CustomErrors::UserNotExist => HttpResponse::NotFound().json("User does not exist"),

            CustomErrors::BadRequest(msg) => HttpResponse::BadRequest().json(msg),

            CustomErrors::DatabaseError(msg) => HttpResponse::InternalServerError().json(msg),

            CustomErrors::InternalError => HttpResponse::InternalServerError().json("Internal server error"),
        }
    }
}
