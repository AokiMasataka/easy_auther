use sqlx::types::uuid;
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct EaClaims {
    pub id: uuid::Uuid,
    pub user_name: String,
    pub is_refresh: bool
}


pub fn sign(
    private_key: &RS256KeyPair,
    id: uuid::Uuid,
    user_name: &str,
    is_refresh: bool
) -> String {
    let claims = EaClaims{
        id,
        user_name: user_name.to_string(),
        is_refresh
    };

    let duration = if is_refresh {
        //Duration::from_days(7)
        Duration::from_mins(2)
    } else {
        Duration::from_mins(1)
    };

    let claims = Claims::with_custom_claims(claims, duration);
    private_key.sign(claims).unwrap()
}
