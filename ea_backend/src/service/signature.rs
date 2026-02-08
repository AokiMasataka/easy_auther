use sqlx::types::uuid;
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;
use crate::core::exceptions::AppError;


#[derive(Serialize, Deserialize)]
pub struct EaClaims {
    pub id: uuid::Uuid,
}


pub fn sign(
    private_key: &RS256KeyPair,
    id: uuid::Uuid,
) -> Result<String, AppError> {
    let claims = EaClaims{id};

    let duration = Duration::from_mins(30);
    let claims = Claims::with_custom_claims(claims, duration);
    match private_key.sign(claims) {
        Ok(token) => Ok(token),
        Err(_) => Err(AppError::Unauthorized)
    }
}
