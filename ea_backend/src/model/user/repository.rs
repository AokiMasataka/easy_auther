use sqlx::{postgres::PgQueryResult, Postgres};

use crate::model::Pgsql;
use super::schema::{User, Identity};


pub async fn create(
    pool: &Pgsql,
    user: &User
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        INSERT INTO users
            (id, group_id, name, pass)
        VALUES
            ($1, $2, $3, $4)
    "#;

    sqlx::query::<Postgres>(query)
        .bind(&user.id)
        .bind(&user.group_id)
        .bind(&user.name)
        .bind(&user.pass)
        .execute(pool)
        .await
}


pub async fn delete(
    pool: &Pgsql,
    user_id: &uuid::Uuid,
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        DELETE FROM users
        WHERE
            id = $1
    "#;

    sqlx::query::<Postgres>(query)
        .bind(user_id)
        .execute(pool)
        .await
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
            users
        WHERE
            name = $1 AND pass = $2
    "#;
    
    sqlx::query_as::<_, Identity>(query)
        .bind(name)
        .bind(pass)
        .fetch_one(pool)
        .await
}





