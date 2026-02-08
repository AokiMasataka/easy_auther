use actix_web::{cookie::{Cookie, SameSite}, web, HttpRequest, HttpResponse};
use serde::{Deserialize, Serialize};
use crate::{core::AppState, service::authorize};


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
    let (token, refresh_token) = authorize::issue_tokens(
        &app_state.db_pool,
        &app_state.key_pair,
        &payload.authorization_code,
        &payload.code_verifier
    ).await.unwrap();

    
    let cookie = Cookie::build("refresh_token", refresh_token)
        .path("/")
        .http_only(true)
        //.secure(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    let response = TokenResponse{ jwt: token };
    HttpResponse::Ok()
        .cookie(cookie)
        .json(response)
}


pub async fn refresh_token(
    app_state: web::Data<AppState>,
    req: HttpRequest
) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(refresh_token) => refresh_token.value().to_string(),
        None => {
            tracing::info!("refresh token is not set");
            return HttpResponse::Unauthorized().finish();
        }
    };


    let (new_token, new_refresh_token) = authorize::refresh(
        &app_state.db_pool,
        &app_state.key_pair,
        &refresh_token
    )
        .await
        .unwrap();
    

    let cookie = Cookie::build("refresh_token", new_refresh_token)
        .path("/")
        .http_only(true)
        //.secure(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    let response = TokenResponse{ jwt: new_token };
    HttpResponse::Ok()
        .cookie(cookie)
        .json(response)
}


pub async fn logout(
    app_state: web::Data<AppState>,
    req: HttpRequest
) -> HttpResponse {
    let refresh_token = match req.cookie("refresh_token") {
        Some(refresh_token) => refresh_token.value().to_string(),
        None => return HttpResponse::Unauthorized().finish()
    };

    authorize::logout(
        &app_state.db_pool,
        &refresh_token
    ).await;

    let cookie = Cookie::build("refresh_token", "")
        .path("/")
        .http_only(true)
        //.secure(true)
        .same_site(SameSite::Lax)
        .secure(false)
        .finish();

    HttpResponse::Ok()
        .cookie(cookie)
        .finish()
}