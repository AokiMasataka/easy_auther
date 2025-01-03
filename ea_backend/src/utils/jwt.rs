use sqlx::types::uuid;
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct GroupClaims{
    pub id: uuid::Uuid,
    pub is_refresh: bool
}


#[derive(Serialize, Deserialize)]
pub struct UserClaims{
    pub id: uuid::Uuid,
    pub is_refresh: bool
}


pub fn create_group_jwt(
    private_key: &RS384KeyPair,
    id: uuid::Uuid,
    is_refresh: bool
) -> String {
    let claims = GroupClaims{id, is_refresh};
    
    let duration = if is_refresh {
        Duration::from_days(7)
    } else {
        Duration::from_mins(30)
    };

    let claims = Claims::with_custom_claims(claims, duration);
    
    private_key.sign(claims).unwrap()
}



pub fn create_user_jwt(
    private_key: &RS384KeyPair,
    id: uuid::Uuid,
    is_refresh: bool
) -> String {
    let claims = UserClaims{id, is_refresh};
    
    let duration = if is_refresh {
        Duration::from_days(7)
    } else {
        Duration::from_mins(30)
    };

    let claims = Claims::with_custom_claims(claims, duration);
    
    private_key.sign(claims).unwrap()
}


pub fn create_private_key() -> RS384KeyPair {
    RS384KeyPair::generate(2048).unwrap()
}
