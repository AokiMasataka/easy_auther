use actix_web::{HttpResponse, web};
use serde::{Deserialize, Serialize};
use crate::{service::authorize, state::AppState};


#[derive(Deserialize)]
pub struct AuthorizeRequest {
    client_id: String,
    email: String,
    password: String,
    redirect_uri: String,
    code_challenge: String,
}


#[derive(Serialize)]
struct AuthorizeResponse {
    authorization_code: String
}


#[derive(Deserialize)]
pub struct TokenRequest {
    authorization_code: String,
    code_verifier: String
}


#[derive(Serialize)]
struct TokenResponse {
    jwt: String
}


pub async fn auth(
    app_state: web::Data<AppState>,
    paylaod: web::Json<AuthorizeRequest>
) -> HttpResponse {

    let authorize_code = authorize::authorize(
        &app_state.db_pool,
        &paylaod.email,
        &paylaod.password,
        &paylaod.code_challenge
    ).await.unwrap();

    if &app_state.config.client_id != &paylaod.client_id {
        return HttpResponse::Forbidden().body("invlid client_id".to_string());
    };

    let response = AuthorizeResponse{
        authorization_code: authorize_code
    };
    HttpResponse::Ok().json(response)
}


pub async fn token(
    app_state: web::Data<AppState>,
    payload: web::Json<TokenRequest>
) -> HttpResponse {
    
    let t = authorize::token(
        &app_state.db_pool,
        &app_state.key_pair,
        &payload.authorization_code,
        &payload.code_verifier
    ).await.unwrap();

    let response = TokenResponse{ jwt: t };
    HttpResponse::Ok().json(response)
}

