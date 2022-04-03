use crate::lib::error::{ErrResponse, ErrType};
use actix_web::{http::header, HttpRequest, HttpResponse};
use std::env;

pub struct Auth;

impl Auth {
  pub fn validate_and_response(req: HttpRequest, res: HttpResponse) -> HttpResponse {
    let req_auth = req.headers().get(header::AUTHORIZATION);
    let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN must be set");

    match req_auth {
      Some(token) => {
        if token.to_str().unwrap() == auth_token {
          res
        } else {
          ErrResponse::new_message(ErrType::Unauthorized, "Invalid Authorization".to_string())
        }
      }
      None => ErrResponse::new_message(ErrType::Unauthorized, "Authorization not set".to_string()),
    }
  }
}
