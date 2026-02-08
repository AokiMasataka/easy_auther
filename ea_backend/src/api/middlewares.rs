use jwt_simple::prelude::*;
use actix_web::{
    body::{MessageBody, EitherBody, BoxBody},
    dev::{ServiceRequest, ServiceResponse},
    middleware::Next,
    Error,
    HttpResponse
};
use base64::{engine::general_purpose, Engine as _};
use std::time::Instant;

use crate::{core::AppState, service::signature::EaClaims};
use serde::Serialize;


#[derive(Serialize)]
struct UnAuthError {
    detail: String
}


pub async fn access_log(
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, Error> {
    let start_time = Instant::now();
    let method = req.method().to_string();
    let uri = req.uri().to_string();

    let res = next.call(req).await?;

    let status = res.status().as_u16();
    let exec_time = format!("{}ms", start_time.elapsed().as_millis());
    tracing::info!(status, method, uri, exec_time);
    Ok(res)
}


fn check_jwt(jwt: &str, key_pair: &RS256KeyPair) -> bool {
    if jwt.starts_with("Bearer") == false {
        return false;
    };

    let jwt = &jwt[7..];

    match key_pair.public_key().verify_token::<EaClaims>(jwt, None) {
        Ok(_) => true,
        Err(_) => false
    }
}


pub async fn validate_jwt<B>(
    req: ServiceRequest,
    next: Next<B>,
) -> Result<ServiceResponse<EitherBody<BoxBody, B>>, Error>
where
    B: MessageBody + 'static, // B は next の持つ元のボディ型
{
    let auth_header = req.headers().get("Authorization");

    let auth_header = match auth_header {
        Some(value) => value.to_str().unwrap_or(""),
        None => {
            let (http_req, _payload) = req.into_parts();
            let res = HttpResponse::Unauthorized()
                .json(UnAuthError{ detail: "Token not set".to_string() })
                .map_into_boxed_body()
                .map_into_left_body();
            return Ok(ServiceResponse::new(http_req, res));
        }
    };

    let app_state = req.app_data::<actix_web::web::Data<AppState>>().unwrap();
    
    if !check_jwt(auth_header, &app_state.key_pair) {
        let (http_req, _payload) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(UnAuthError{ detail: "invalid token".to_string() })
            .map_into_boxed_body()
            .map_into_left_body();
        return Ok(ServiceResponse::new(http_req, res));
    }

    let res = next.call(req).await?;
    Ok(res.map_into_right_body())
}


fn check_secret(basic: &str, client_id: &str, client_secret: &str) -> bool {
    let raw = format!("{}:{}", client_id, client_secret);
    let encoded = general_purpose::STANDARD.encode(raw);
    let expected = format!("Basic {}", encoded);
    basic == expected
}


pub async fn validate_secret<B>(
    req: ServiceRequest,
    next: Next<B>
) -> Result<ServiceResponse<EitherBody<BoxBody, B>>, Error>
where
    B: MessageBody + 'static
{
    let auth_header = req.headers().get("Authorization");

    let auth_header = match auth_header {
        Some(auth_header) => auth_header.to_str().unwrap(),
        None=> {
            let (http_req, _payload) = req.into_parts();
            let res = HttpResponse::Unauthorized()
                .json(UnAuthError{ detail: "Secret not set".to_string() })
                .map_into_boxed_body()
                .map_into_left_body();
            return Ok(ServiceResponse::new(http_req, res));
        },
    };

    let app_state = req.app_data::<actix_web::web::Data<AppState>>().unwrap();

    if !(check_secret(auth_header, &app_state.config.client_id, &app_state.config.client_secret)) {
        let (http_req, _payload) = req.into_parts();
        let res = HttpResponse::Unauthorized()
            .json(UnAuthError{ detail: "Secret not set".to_string() })
            .map_into_boxed_body()
            .map_into_left_body();
        return Ok(ServiceResponse::new(http_req, res));
    }
    
    let res = next.call(req).await?;
    Ok(res.map_into_right_body())
}

