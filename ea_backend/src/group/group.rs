use sqlx::{postgres::PgQueryResult, Postgres, FromRow, types::uuid};
use serde::{Serialize, Deserialize};
use jwt_simple::prelude::*;

use crate::model::config::Pgsql;


#[derive(FromRow, Serialize, Deserialize)]
pub struct Group {
    pub id: uuid::Uuid,
    pub name: String,
    pub pass: String,
    pub private_key: String,
    pub public_key: String,
}

#[derive(FromRow)]
pub struct LoginSchema {
    pub id: uuid::Uuid,
    pub name: String,
}


impl Group {
    pub fn new(name: String, pass: String) -> Self {
        let id = uuid::Uuid::new_v4();
        
        let private_key = RS384KeyPair::generate(2048)
            .unwrap()
            .to_pem()
            .unwrap();

        let public_key = RS384KeyPair::from_pem(&private_key)
            .unwrap()
            .public_key()
            .to_pem()
            .unwrap();

        Group{
            id,
            name,
            pass,
            private_key,
            public_key,
        }
    }
}


pub async fn create_group(
    pool: &Pgsql,
    group: &Group
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        INSERT INTO groups
            (id, name, pass, private_key, public_key)
        VALUES
            ($1, $2, $3, $4, $5)
    "#;

    let result = sqlx::query::<Postgres>(query)
        .bind(&group.id)
        .bind(&group.name)
        .bind(&group.pass)
        .bind(&group.private_key)
        .bind(&group.public_key)
        .execute(pool)
        .await;

    return result;
}

pub async fn delete_group(
    pool: &Pgsql,
    group_id: &uuid::Uuid
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        DELETE FROM groups
        WHERE
            id = $1
    "#;
    
    let res = sqlx::query::<Postgres>(query)
        .bind(group_id)
        .execute(pool)
        .await;

    return res;
}

pub async fn get_public_key(
    pool: &Pgsql,
    group_id: &uuid::Uuid
) -> RS384PublicKey {
    let query = r#"
        SELECT
            public_key
        FROM
            groups
        WHERE
            id = $1
        "#;

    let pem: String = sqlx::query_scalar(query)
        .bind(group_id)
        .fetch_one(pool)
        .await
        .unwrap();
    
    RS384PublicKey::from_pem(&pem).unwrap()
}

pub async fn get_private_key(
    pool: &Pgsql,
    group_id: &uuid::Uuid
) -> RS384KeyPair {
    let query = r#"
        SELECT
            private_key
        FROM
            groups
        WHERE
            id = $1
        "#;

    let pem: String = sqlx::query_scalar(query)
        .bind(group_id)
        .fetch_one(pool)
        .await
        .unwrap();

    RS384KeyPair::from_pem(&pem).unwrap()
}


pub async fn login(
    pool: &Pgsql,
    name: &str,
    pass: &str
) -> Result<LoginSchema, sqlx::Error> {
    let query = r#"
        SELECT
            id, name
        FROM
            groups
        WHERE
            name = $1 AND pass = $2
    "#;

    sqlx::query_as::<_, LoginSchema>(query)
        .bind(name)
        .bind(pass)
        .fetch_one(pool)
        .await
}
