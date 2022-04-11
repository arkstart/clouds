use actix_web::{dev::ServiceRequest, Error};
use actix_web_httpauth::extractors::{bearer, AuthenticationError};
use std::env;

pub async fn check_bearer_token(
  req: ServiceRequest,
  credentials: bearer::BearerAuth,
) -> Result<ServiceRequest, Error> {
  if req.method() == "POST" || req.method() == "PUT" {
    let auth_token = env::var("AUTHORIZATION_TOKEN").expect("AUTHORIZATION_TOKEN is not set");
    if credentials.token() == auth_token {
      Ok(req)
    } else {
      let config = req
        .app_data::<bearer::Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default)
        .scope("Unauthorized");

      Err(AuthenticationError::from(config).into())
    }
  } else {
    Ok(req)
  }
}
