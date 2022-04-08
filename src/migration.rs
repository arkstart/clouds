use crate::lib::error::*;
use actix_web::{http::header, HttpRequest, HttpResponse};
use std::env;
use std::process::Command;

pub async fn migrate(req: HttpRequest) -> HttpResponse {
  let req_auth = req.headers().get(header::AUTHORIZATION);
  let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN must be set");

  match req_auth {
    Some(token) => {
      if token.to_str().unwrap() == auth_token {
        let cmd = Command::new("diesel")
          .args(["migration", "run"])
          .spawn()
          .expect("failed to execute process");
        HttpResponse::Ok().body(format!("{:?}", cmd.stdout))
      } else {
        ErrResponse::new_message(ErrType::Unauthorized, "Invalid Authorization".to_string())
      }
    }
    None => ErrResponse::new_message(ErrType::Unauthorized, "Authorization not set".to_string()),
  }
}
