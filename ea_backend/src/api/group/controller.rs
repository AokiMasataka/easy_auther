use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::types::uuid;
use jwt_simple::prelude::*;

use crate::model::{Pgsql, group};
use crate::utils::jwt;
use super::schema::{
    CreateRequest,
    CreateResponse,
    LoginRequest,
    LoginResponse,
    RefreshResponse,
    GetAllResponse
};


pub async fn get_users(
    pool: web::Data<Pgsql>,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
        
    match group::get_users(&pool, &path.into_inner()).await {
        Ok(users) => HttpResponse::Ok().json(GetAllResponse{users}),
        Err(e) => {
            println!("Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn create(
    pool: web::Data<Pgsql>,
    payload: web::Json<CreateRequest>
) -> HttpResponse {
    println!("[group create] name: {}", &payload.name);
    let new_group = group::Group::new(
        payload.name.clone(),
        payload.pass.clone()
    );

    match group::create(&pool, &new_group).await {
        Ok(_) => HttpResponse::Created()
            .json(CreateResponse{id: new_group.id}),
        Err(e) => {
            println!("[group create] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn update(
    pool: web::Data<Pgsql>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CreateRequest>
) -> HttpResponse {
    let group_id = path.into_inner();
    let mut update_group = match group::get(&pool, &group_id).await {
        Ok(g) => g,
        Err(sqlx::Error::RowNotFound) => {
            return HttpResponse::NotFound().finish()
        },
        Err(_) => {
            return HttpResponse::InternalServerError().finish()
        }
    };
    
    update_group.update(payload.name.clone(), payload.pass.clone());

    match group::update(&pool, &update_group).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(e) => {
            println!("[group update] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}


pub async fn delete(
    pool: web::Data<Pgsql>,
    group_id: web::Path<uuid::Uuid>
) -> HttpResponse {
    println!("[group delete]");

    match group::delete(&pool, &group_id.into_inner()).await {
        Ok(_) => HttpResponse::NoContent().finish(),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().finish(),
        Err(e) => {
            println!("[group delete] Error: {}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

pub async fn login(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    payload: web::Json<LoginRequest>
) -> HttpResponse {
    println!("[group login] name: {}", payload.name);

    let identity = match group::login(
        &pool, &payload.name, &payload.pass
    ).await {
        Ok(identity) => identity,
        Err(e) => {
            println!("[group login] Error: {}", e);
            return HttpResponse::Unauthorized().finish();
        }
    };
    
    let jwt = jwt::create_group_jwt(&private_key, identity.id, false);
    let refresh_jwt = jwt::create_group_jwt(&private_key, identity.id, true);
    HttpResponse::Ok().json(
        LoginResponse{id: identity.id, jwt, refresh_jwt}
    )
}

pub async fn refresh(
    private_key: web::Data<RS384KeyPair>,
    req: HttpRequest
) -> HttpResponse {
    let jwt = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let claims = private_key
        .public_key()
        .verify_token::<jwt::GroupClaims>(&jwt, None).unwrap();

    if !claims.custom.is_refresh {
        return  HttpResponse::Unauthorized().body("Access tokens are not allowed");
    }

    let new_jwt = jwt::create_group_jwt(&private_key, claims.custom.id, false);
        
    HttpResponse::Ok().json(RefreshResponse{jwt: new_jwt})
}
