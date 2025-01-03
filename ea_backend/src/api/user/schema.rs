use sqlx::types::uuid;
use serde::{Serialize, Deserialize};

use crate::model::user;


#[derive(Deserialize)]
pub struct CreateRequest {
    pub name: String,
    pub pass: String
}

#[derive(Serialize)]
pub struct CreateResponse {
    pub id: uuid::Uuid
}

#[derive(Deserialize)]
pub struct LoginRequest {
    pub name: String,
    pub pass: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub id: uuid::Uuid,
    pub jwt: String,
    pub refresh_jwt: String
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub jwt: String
}

#[derive(Deserialize)]
pub struct RefreshRequest {
    pub refresh_jwt: String
}

#[derive(Serialize)]
pub struct RefreshResponse {
    pub jwt: String
}


#[derive(Serialize)]
pub struct GetAllResponse {
    pub users: Vec<user::UserCardSchema>
}

