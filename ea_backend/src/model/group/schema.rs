use sqlx::{FromRow, types::uuid};
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;


#[derive(FromRow, Serialize, Deserialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub pass: String,
    pub private_key: String,
    pub public_key: String
}

impl Group {
    pub fn new(name: String, pass: String) -> Self {
        let id = uuid::Uuid::new_v4();

        let private_key = RS384KeyPair::generate(2048)
            .unwrap();

        let private_pem = private_key
            .to_pem()
            .unwrap();

        let public_pem = private_key
            .public_key()
            .to_pem()
            .unwrap();
        
        Group{
            id,
            name,
            pass,
            private_key: private_pem,
            public_key: public_pem
        }
    }
}

#[derive(FromRow, Serialize, Deserialize)]
pub struct Identity {
    pub id: uuid::Uuid,
    pub name: String,
}
