use sqlx::{FromRow, types::uuid};
use serde::{Serialize, Deserialize};


#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: uuid::Uuid,
    pub group_id: uuid::Uuid,
    pub name: String,
    pub pass: String
}

impl User{
    pub fn new(
        group_id: uuid::Uuid,
        name: String,
        pass: String
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        User{id, group_id, name, pass}
    }
}


#[derive(FromRow, Serialize)]
pub struct Identity {
    pub id: uuid::Uuid,
    pub name: String
}
