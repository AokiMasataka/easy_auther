use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::types::uuid;
use jwt_simple::prelude::*;

use crate::model::Pgsql;
use crate::model::{user, group};
use crate::utils::jwt;
use super::schema::{
    GetAllResponse,
    CreateRequest,
    CreateResponse,
    LoginRequest,
    LoginResponse,
    RefreshRequest,
    RefreshResponse,
    VerifyRequest
};


pub async fn get_all(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    group_id: web::Path<uuid::Uuid>,
    request: HttpRequest
) -> HttpResponse {
    let _ = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user get all authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    match user::get_all(&pool, &group_id).await {
        Ok(users) => HttpResponse::Ok().json(GetAllResponse{users}),
        Err(e) => {
            println!("[user get all] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn create(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<CreateRequest>,
    request: HttpRequest,
) -> HttpResponse {
    println!("[user create]");
    let claims = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user create authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let new_user = user::User::new(
        claims.custom.id,
        payload.name.clone(),
        payload.pass.clone()
    );

    match user::create(&pool, &new_user).await {
        Ok(_) => HttpResponse::Created()
            .json(CreateResponse{id: new_user.id}),
        Err(e) => {
            println!("[user create] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn delete(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
    request: HttpRequest
) -> HttpResponse {
    let _ = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user delete authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let (_, user_id) = path.into_inner();

    match user::delete(&pool, &user_id).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("[user delete] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn login(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<LoginRequest>,
    request: HttpRequest
) -> HttpResponse {
    let claims = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user login authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let identity = match user::login(&pool, &payload.name, &payload.pass).await {
        Ok(identity) => identity,
        Err(e) => {
            println!("[user login] Error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let key = group::get_private_key(&pool, &claims.custom.id).await;
    let jwt = jwt::create_user_jwt(&key, identity.id, false);
    let refresh_jwt = jwt::create_user_jwt(&key, identity.id, true);

    HttpResponse::Ok().json(
        LoginResponse{id: identity.id, jwt, refresh_jwt}
    )
}


pub async fn refresh(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<RefreshRequest>,
    request: HttpRequest
) -> HttpResponse {
    let claims = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user refresh authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let key = group::get_public_key(&pool, &claims.custom.id).await;

    match key.verify_token::<jwt::UserClaims>(&payload.refresh_jwt, None) {
        Ok(user_claims) => {
            let private_key = group::get_private_key(&pool, &claims.custom.id).await;
            let response = RefreshResponse{
                jwt: jwt::create_user_jwt(&private_key, user_claims.custom.id, false)
            };
            HttpResponse::Ok().json(response)
        },
        Err(_) => HttpResponse::Unauthorized().finish()
    }
}

pub async fn verify(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<VerifyRequest>,
    request: HttpRequest
) -> HttpResponse {
    let claims = match authorization(&request, &private_key) {
        Ok(claims) => claims,
        Err(e) => {
            println!("[user verfiy authorization] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };

    let key = group::get_public_key(&pool, &claims.custom.id).await;
    match key.verify_token::<jwt::UserClaims>(&payload.jwt, None) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("[user verfiy] Error: {}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}


fn authorization(
    request: &HttpRequest, private_key: &RS384KeyPair
) -> Result<JWTClaims<jwt::UserClaims>, jwt_simple::Error> {
    let jwt = request
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap();

    private_key.public_key().verify_token::<jwt::UserClaims>(jwt, None)
}
