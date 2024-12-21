use jwt_simple::prelude::{RS384KeyPair, RSAPublicKeyLike};
use sqlx::types::uuid;
use actix_web::{web, HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};

use crate::group::{token, group};
use crate::model::config::Pgsql;

#[derive(Deserialize)]
pub struct CreateGroupRequest {
    name: String,
    pass: String
}


#[derive(Serialize)]
struct CreateGroupResponse {
    id: uuid::Uuid
}

#[derive(Deserialize)]
pub struct LoginRequest {
    name: String,
    pass: String
}

#[derive(Serialize)]
struct LoginResponse {
    id: uuid::Uuid,
    token: String,
    refresh: String
}

#[derive(Serialize)]
struct RefreshTokenResponse {
    token: String
}


pub async fn create_group(
    pool: web::Data<Pgsql>,
    req: web::Json<CreateGroupRequest>
) -> HttpResponse {
    println!("crteae group: {}", &req.name);
    let new_group = group::Group::new(req.name.clone(), req.pass.clone());
    
    match group::create_group(&pool, &new_group).await {
        Ok(_) => HttpResponse::Created()
            .json(CreateGroupResponse{id: new_group.id}),
        Err(e) => {
            println!("[cretae group] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}


pub async fn delete_group(
    pool: web::Data<Pgsql>,
    group_id: web::Path<uuid::Uuid>
) -> HttpResponse {
    let group_id = group_id.into_inner();
    println!("delete group: {}", group_id);
    
    match group::delete_group(&pool, &group_id).await {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound().body("group not found"),
        Err(e) => {
            println!("[delete group] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}


pub async fn login_group(
    pool: web::Data<Pgsql>,
    private_key: web::Data<RS384KeyPair>,
    req: web::Json<LoginRequest>,
) -> HttpResponse {
    println!("name: {}", req.name);
    let result = group::login(&pool, &req.name, &req.pass).await;
    
    let schema = match result {
        Ok(s) => s,
        Err(e) => {
            println!("login group err: {}", e);
            return HttpResponse::NotFound().json("Not Found Error");
        }
    };

    let jwt = token::create_jwt(
        &private_key,
        schema.id,
        &schema.name,
        false
    );
    
    let refresh_jwt = token::create_jwt(
        &private_key,
        schema.id,
        &schema.name,
        true
    );
    HttpResponse::Ok().json(
        LoginResponse{id: schema.id, token: jwt, refresh: refresh_jwt}
    )
}


pub async fn refresh_token(
    private_key: web::Data<RS384KeyPair>,
    req: HttpRequest
) -> HttpResponse {
    let token = req
        .headers()
        .get("Authorization")
        .unwrap()
        .to_str()
        .unwrap()[7..]
        .to_string();

    let claims = private_key
        .public_key()
        .verify_token::<token::GroupClaims>(&token, None).unwrap();
    
    if ! claims.custom.is_refresh {
        return HttpResponse::BadRequest().body("Access tokens are not allowed");
    };

    let new_jwt = token::create_jwt(
        &private_key,
        claims.custom.id,
        &claims.custom.name,
        false
    );

    HttpResponse::Ok().json(RefreshTokenResponse{token: new_jwt})
}

