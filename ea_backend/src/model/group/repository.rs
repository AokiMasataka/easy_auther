use sqlx::{postgres::PgQueryResult, Postgres};
use jwt_simple::prelude::*;

use crate::model::Pgsql;
use crate::model::group::schema::{Group, Identity};


pub async fn create(
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

pub async fn update(
    pool: &Pgsql,
    group: &Group
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        UPDATE groups
        SET
            name = '$2', pass = '$3', private_key = '$4', public_key = '$5'
        WHERE
            id = $1
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

pub async fn delete(
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
pub async fn _get() {}

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
) -> Result<Identity, sqlx::Error> {
    let query = r#"
        SELECT
            id, name
        FROM
            groups
        WHERE
            name = $1 AND pass = $2
    "#;

    sqlx::query_as::<_, Identity>(query)
        .bind(name)
        .bind(pass)
        .fetch_one(pool)
        .await
}
