use actix_web::{http::StatusCode, HttpResponse};
use serde::Serialize;

#[derive(Debug, Clone)]
pub enum ErrorType {
  BadRequest,
  InternalServerError,
}

#[derive(Debug, Clone, Serialize)]
pub struct ErrorResponse {
  code: u16,
  error: String,
  message: String,
}

impl ErrorResponse {
  pub fn new(error_type: ErrorType, error: String) -> HttpResponse {
    let error_res = ErrorResponse {
      code: ErrorResponse::status_code(error_type.clone()).as_u16(),
      error: error,
      message: ErrorResponse::error_message(error_type.clone()),
    };

    error_res.return_httpresponse(error_type)
  }

  pub fn new_message(error_type: ErrorType, message: String) -> HttpResponse {
    let error_res = ErrorResponse {
      code: ErrorResponse::status_code(error_type.clone()).as_u16(),
      error: "".to_string(),
      message: message,
    };

    error_res.return_httpresponse(error_type)
  }

  fn status_code(error_type: ErrorType) -> StatusCode {
    match error_type {
      ErrorType::BadRequest => StatusCode::BAD_REQUEST,
      ErrorType::InternalServerError => StatusCode::INTERNAL_SERVER_ERROR,
    }
  }

  fn error_message(error_type: ErrorType) -> String {
    match error_type {
      ErrorType::BadRequest => "Bad Request".to_string(),
      ErrorType::InternalServerError => "Internal Server Error".to_string(),
    }
  }

  fn return_httpresponse(self, error_type: ErrorType) -> HttpResponse {
    match error_type {
      ErrorType::BadRequest => HttpResponse::BadRequest().json(self),
      ErrorType::InternalServerError => HttpResponse::InternalServerError().json(self),
    }
  }
}
