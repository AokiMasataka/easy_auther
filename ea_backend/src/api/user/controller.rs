use actix_web::{HttpResponse, web};
use sqlx::types::uuid;
use jwt_simple::prelude::*;

use crate::model::Pgsql;
use crate::model::{user, group};
use crate::utils::jwt;
use super::schema::{
    CreateRequest,
    CreateResponse,
    LoginRequest,
    LoginResponse,
    RefreshRequest,
    RefreshResponse,
    VerifyRequest
};


pub async fn create(
    pool: web::Data<Pgsql>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CreateRequest>
) -> HttpResponse {
    let payload = payload.into_inner();
    let new_user = user::User::new(
        path.into_inner(),
        payload.name,
        payload.pass
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
    path: web::Path<(uuid::Uuid, uuid::Uuid)>,
) -> HttpResponse {
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
    path: web::Path<uuid::Uuid>,
    payload: web::Json<LoginRequest>,
) -> HttpResponse {
    let identity = match user::login(&pool, &payload.name, &payload.pass).await {
        Ok(identity) => identity,
        Err(e) => {
            println!("[user login] Error: {}", e);
            return HttpResponse::InternalServerError().finish();
        }
    };

    let key = group::get_private_key(&pool, &path.into_inner()).await;
    let jwt = jwt::create_user_jwt(&key, identity.id, false);
    let refresh_jwt = jwt::create_user_jwt(&key, identity.id, true);

    HttpResponse::Ok().json(
        LoginResponse{id: identity.id, jwt, refresh_jwt}
    )
}


pub async fn refresh(
    pool: web::Data<Pgsql>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<RefreshRequest>,
) -> HttpResponse {
    let group_id = path.into_inner();
    let key = group::get_public_key(&pool, &group_id).await;

    match key.verify_token::<jwt::UserClaims>(&payload.refresh_jwt, None) {
        Ok(user_claims) => {
            let private_key = group::get_private_key(&pool, &group_id).await;
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
    path: web::Path<uuid::Uuid>,
    payload: web::Json<VerifyRequest>,
) -> HttpResponse {
    let key = group::get_public_key(&pool, &path.into_inner()).await;
    match key.verify_token::<jwt::UserClaims>(&payload.jwt, None) {
        Ok(_) => HttpResponse::Ok().finish(),
        Err(e) => {
            println!("[user verfiy] Error: {}", e);
            HttpResponse::Unauthorized().finish()
        }
    }
}

