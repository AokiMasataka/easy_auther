use actix_web::{HttpRequest, HttpResponse};
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::{RS384KeyPair, RSAPublicKeyLike};
use sqlx::types::uuid;

use create::user::user;
use create::model::config::Pgsql;


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
        Err(e) => {
            HttpResponse::Unauthorized().body("")
        }
    }

    let new_user = user::User::new(
        group_id,
        payload.name.clone(),
        payload.pass.clone()
    );

    match user::create_user(&pool, &new_user).await {
        Ok(_) => HttpRequest::Created()
            .json(CreateGroupResponse{id: new_user.id}),
        Err(e) => {
            println!("[user create] Error: {}", e);
            HttpResponse::InternalServerError().body("")
        }
    }
}


fn authorization(
    req: &HttpRequest,
    private_key: &RS384KeyPair
) -> Result<GroupClaims, jwt_simple::error::Error> {
    let token = req.headers().get("Authorization").unwrap();
    let token = token.to_str().unwrap();

    let claims = private_key.verify_token::<GroupClaims>(token)
}