use actix_web::{
    error::{BlockingError, ResponseError},
    http::{header, StatusCode},
    HttpResponse, HttpResponseBuilder,
};
use derive_more::Display;
use diesel::{
    r2d2::PoolError,
    result::{DatabaseErrorKind, Error as DBError},
};
use serde::{Deserialize, Serialize};
use std::ops::Deref;

#[macro_use]
extern crate lazy_static;
pub mod awc;
mod config;
pub mod constants;
mod encrypt;
mod authorization;
mod mail;
mod validate;
mod pagination;
pub use self::{config::*, encrypt::*, authorization::*, mail::*,pagination::*,validate::*};

#[derive(Debug, Display, PartialEq, Serialize)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
    NotFound(String),
    PaymentRequired(String),
    DataBaseError(String),
    Unauthorized(String),
    #[display(fmt = "")]
    ValidateError(Vec<String>),
}

/// User-friendly error messages
#[derive(Debug, Deserialize, Serialize)]
pub struct ErrorResponse<T> {
    errors: Vec<T>,
}

impl<String> Deref for ErrorResponse<String> {
    type Target = Vec<String>;

    fn deref(&self) -> &Self::Target {
        &self.errors
    }
}

/// custom error
impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::NotFound(_) => StatusCode::NOT_FOUND,
            Error::Unauthorized(_) => StatusCode::UNAUTHORIZED,
            Error::PaymentRequired(_) => StatusCode::PAYMENT_REQUIRED,
            _ => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponseBuilder::new(self.status_code())
            .insert_header((header::CONTENT_TYPE, "text/html; charset=utf-8"))
            .body(self.to_string())
    }
}

impl From<Vec<String>> for ErrorResponse<String> {
    fn from(errors: Vec<String>) -> Self {
        ErrorResponse { errors }
    }
}

/// Convert DBErrors to ServiceErrors
impl From<DBError> for Error {
    fn from(error: DBError) -> Error {
        // Right now we just care about UniqueViolation from diesel
        // But this would be helpful to easily map errors as our app grows
        match error {
            DBError::DatabaseError(kind, info) => {
                if let DatabaseErrorKind::UniqueViolation = kind {
                    let message = info.details().unwrap_or_else(|| info.message()).to_string();
                    return Error::BadRequest(message);
                }
                Error::InternalServerError("Unknown database error".into())
            }
            _ => Error::InternalServerError("Unknown database error".into()),
        }
    }
}

/// Convert PoolErrors to ServiceErrors
impl From<PoolError> for Error {
    fn from(error: PoolError) -> Error {
        Error::DataBaseError(error.to_string())
    }
}

/// Convert BlockingError to ServiceErrors
impl From<BlockingError> for Error {
    fn from(error: BlockingError) -> Error {
        Error::InternalServerError(error.to_string())
    }
}
