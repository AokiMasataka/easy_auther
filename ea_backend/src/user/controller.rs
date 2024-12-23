use actix_web::{HttpRequest, HttpResponse, web};
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;
use sqlx::types::uuid;

use crate::user::user;
use crate::group::token::GroupClaims;
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


fn authorization(
    req: &HttpRequest,
    private_key: &RS384KeyPair
) -> Result<JWTClaims<GroupClaims>, jwt_simple::Error> {
    let token = req.headers().get("Authorization").unwrap();
    let token = token.to_str().unwrap();

    private_key.public_key().verify_token::<GroupClaims>(token, None)
}
