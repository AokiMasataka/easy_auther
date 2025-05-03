use jwt_simple::prelude::*;
use actix_web::{
    HttpResponse,
    body::EitherBody,
    dev::{self, Service, Transform},
    dev::{ServiceRequest, ServiceResponse},
    Error,
};

use std::future::{Ready, ready};
use futures_util::future::LocalBoxFuture;
use super::jwt::EaClaims;

pub struct Authorize;

impl<S, B> Transform<S, ServiceRequest> for Authorize
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthorizeMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthorizeMiddleware { service }))
    }
}
pub struct AuthorizeMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for AuthorizeMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    dev::forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {

        let jwt = if !req.headers().contains_key("Authorization") {
            println!("[auth middle]: Unauthorized");

            let res = HttpResponse::Unauthorized().body("Unauthorized").map_into_right_body();
            let (req, _) = req.into_parts();
            return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
        } else {
            req.headers().get("Authorization").unwrap().to_str().unwrap()
        };
        
        let mut options = VerificationOptions::default();
        options.time_tolerance = Some(Duration::from_mins(1)); 
        let private_key = req.app_data::<actix_web::web::Data<RS384KeyPair>>().unwrap();
        match private_key.public_key().verify_token::<EaClaims>(&jwt, Some(options)){
            Ok(_) => {
                let res = self.service.call(req);
                Box::pin(async move {
                    res.await.map(ServiceResponse::map_into_left_body)
                })
            },
            Err(_) => {
                let res = HttpResponse::Unauthorized().body("Unauthorized").map_into_right_body();
                let (req, _) = req.into_parts();
                return Box::pin(async { Ok(ServiceResponse::new(req, res)) });
            }
        }

    }
}
