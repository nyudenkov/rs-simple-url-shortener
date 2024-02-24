use actix_web::dev::ServiceRequest;
use actix_web::Error;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use std::env;

pub async fn validator(
    req: ServiceRequest,
    auth: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {
    let secret_token = env::var("AUTH_TOKEN").expect("AUTH_TOKEN must be set");
    match auth.token() {
        token if token == secret_token => Ok(req),
        _ => Err((actix_web::error::ErrorUnauthorized("Invalid token"), req)),
    }
}
