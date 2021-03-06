use std::fmt;
use warp::reject::Reject;

#[derive(Debug)]
pub enum ErrorType {
    NotFound,
    Internal,
    BadRequest,
}

#[derive(Debug)]
pub struct AppError {
    pub err_type: ErrorType,
    pub message: String,
}

impl AppError {
    pub fn new(message: &str, err_type: ErrorType) -> AppError {
        AppError { message: message.to_string(), err_type }
    }

    // ErrorType will help us to differentiate between different kinds of errors and map them properly to HTTP status codes in the to_http_status() method.
    pub fn to_http_status(&self) -> warp::http::StatusCode {
        match self.err_type {
            ErrorType::NotFound => warp::http::StatusCode::NOT_FOUND,
            ErrorType::Internal => warp::http::StatusCode::INTERNAL_SERVER_ERROR,
            ErrorType::BadRequest => warp::http::StatusCode::BAD_REQUEST,
        }
    }

    // We will also need to convert errors from Diesel to our AppError with from_diesel_err():
    pub fn from_diesel_err(err: diesel::result::Error, context: &str) -> AppError {
        AppError::new(
            format!("{}: {}", context, err.to_string()).as_str(),
            match err {
                diesel::result::Error::DatabaseError(db_err, _) => {
                    match db_err {
                        diesel::result::DatabaseErrorKind::UniqueViolation => ErrorType::BadRequest, _ => ErrorType::Internal,
                    }
                }
                diesel::result::Error::NotFound => ErrorType::NotFound, _ => {
                    ErrorType::Internal
                }
            },
        )
    }
}

impl std::error::Error for AppError {}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl Reject for AppError {}