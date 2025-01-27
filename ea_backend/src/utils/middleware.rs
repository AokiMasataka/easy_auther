use jwt_simple::prelude::*;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    error::ErrorUnauthorized,
    middleware::Next,
    Error,
};

use super::jwt::GroupClaims;


pub async fn authorize_middleware(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let jwt = if !req.headers().contains_key("X-Authorization") {
        println!("[auth middle]: Unauthorized");
        return Err(ErrorUnauthorized("Unauthorized: Missing Auth header"));
    } else {
        req.headers().get("X-Authorization").unwrap().to_str().unwrap()
    };

    let private_key = req.app_data::<actix_web::web::Data<RS384KeyPair>>().unwrap();
    match private_key.public_key().verify_token::<GroupClaims>(jwt, None){
        Ok(_) => next.call(req).await,
        Err(e) => Err(ErrorUnauthorized(e.to_string()))
    }
}
