use sqlx::types::uuid;
use serde::{Deserialize, Serialize};
use jwt_simple::prelude::*;


#[derive(Serialize, Deserialize)]
pub struct GroupClaims {
    pub id: uuid::Uuid,
    pub name: String,
    pub is_refresh: bool
}


pub fn create_jwt(
    private_key: &RS384KeyPair,
    id: uuid::Uuid,
    name: &str,
    is_refresh: bool
) -> String {
    let group_claims = GroupClaims{
        id, name: name.to_string(), is_refresh
    };
    
    let duration = if is_refresh {
        Duration::from_days(7)
    } else {
        Duration::from_mins(30)
    };

    let claims = Claims::with_custom_claims(group_claims, duration)
        .with_issuer("EzAuth");

    private_key.sign(claims).unwrap()
}


pub fn create_private_key() -> RS384KeyPair {
    let private_key = RS384KeyPair::generate(2048)
        .unwrap();
    return private_key;
}
