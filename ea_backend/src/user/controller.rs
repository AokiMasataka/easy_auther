use std::process::id;

use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;
use sqlx::types::uuid;

use crate::group;
use crate::user::user;
use crate::model::config::Pgsql;


#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    pass: String
}

#[derive(Serialize)]
struct CreateUserResponse {
    id: uuid::Uuid,
}

#[derive(Deserialize)]
pub struct LoginUserRequest {
    name: String,
    pass: String
}

#[derive(Serialize)]
pub struct LoginUserResponse {
    id: uuid::Uuid,
    token: String,
    refresh_token: String
}

#[derive(Serialize)]
struct GetAllResponse {
    users: Vec<user::UserCardSchema>
}


#[derive(Deserialize)]
pub struct VerifyRequest {
    token: String
}


#[derive(Serialize)]
pub struct RefreshResponse {
    token: String
}

pub async fn get_users(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    group_id: web::Path<uuid::Uuid>,
    req: HttpRequest
) -> HttpResponse {
    println!("[user get all]");
    let _claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };

    match user::get_users(&pool, &group_id).await {
        Ok(users) => HttpResponse::Ok().json(GetAllResponse{users}),
        Err(e) => {
            println!("[user get all] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}

pub async fn create_user(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<CreateUserRequest>,
    req: HttpRequest,
) -> HttpResponse {
    println!("[user create] name: {}", &payload.name);
    let claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(e) => return HttpResponse::Unauthorized().body(e.to_string()),
    };

    let new_user = user::User::new(
        claims.custom.id,
        payload.name.clone(),
        payload.pass.clone()
    );

    match user::create_user(&pool, &new_user).await {
        Ok(_) => HttpResponse::Created()
            .json(CreateUserResponse{id: new_user.id}),
        Err(e) => {
            println!("[user create] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}


pub async fn delete_user(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    req: HttpRequest
) -> HttpResponse {

    let _claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().body("")
    };

    let (_group_id, user_id) = path.into_inner();

    match user::delete_user(&pool, &user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("[user delete] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}

pub async fn login(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<LoginUserRequest>,
    req: HttpRequest,
) -> HttpResponse {
    let claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().finish(),
    };
    
    let schema = match user::login(
        &pool,
        &payload.name,
        &payload.pass
    ).await {
        Ok(s) => s,
        Err(e) => {
            println!("[user login] Error: {}", e);
            return HttpResponse::InternalServerError().body("");
        }
    };

    let key = group::group::get_private_key(&pool, &claims.custom.id).await;
    let jwt = group::token::create_jwt(&key, schema.id, &schema.name, false);
    let refresh_jwt = group::token::create_jwt(&key, schema.id, &schema.name, true);

    HttpResponse::Ok().json(
        LoginUserResponse{id: schema.id, token: jwt, refresh_token: refresh_jwt}
    )
}


pub async fn verify(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Data<VerifyRequest>,
    req: HttpRequest
) -> HttpResponse {
    let group_claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };

    let key = group::group::get_public_key(&pool, &group_claims.custom.id).await;
    match key.verify_token::<group::token::GroupClaims>(&payload.token, None) {
        Ok(_) => HttpResponse::Ok().body(""),
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}


pub async fn refresh(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<VerifyRequest>,
    req: HttpRequest
) -> HttpResponse {
    let claims = match authorization(&req, &private_key) {
        Ok(claims) => claims,
        Err(_) => return HttpResponse::Unauthorized().finish()
    };
    
    let key = group::group::get_public_key(&pool, &claims.custom.id).await;

    match key.verify_token::<group::token::GroupClaims>(&payload.token, None){
        Ok(user_claims) => {
            let pri_key = group::group::get_private_key(&pool, &claims.custom.id).await;
            let response = RefreshResponse{
                token: group::token::create_jwt(&pri_key, user_claims.custom.id, &user_claims.custom.name, false)
            };
            HttpResponse::Ok().json(response)
        },
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}


fn authorization(
    req: &HttpRequest,
    private_key: &RS384KeyPair
) -> Result<JWTClaims<group::token::GroupClaims>, jwt_simple::Error> {
    let token = req.headers().get("Authorization").unwrap();
    let token = token.to_str().unwrap();

    private_key.public_key().verify_token::<group::token::GroupClaims>(token, None)
}

