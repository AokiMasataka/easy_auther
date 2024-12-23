use sqlx::{postgres::PgQueryResult, Postgres, FromRow, types::uuid};
use serde::{Serialize, Deserialize};
use crate::model::config::Pgsql;


#[derive(FromRow, Serialize, Deserialize)]
pub struct User {
    id: uuid::Uuid,
    group_id: uuid::Uuid,
    name: String,
    pass: String
}

impl User {
    pub fn new(
        group_id: uuid::Uuid,
        name: String,
        pass: String
    ) -> Self {
        let id = uuid::Uuid::new_v4();
        User{id, group_id, name, pass}
    }
}

pub async fn get_users(
    pool: &Pgsql,
    group_id: &uuid::Uuid
) -> Result<Vec<User>, sqlx::Error> {
    let query = r#"
        SELECT
            id, group_id, name
        FROM
            users
        WHERE
            group_id = $1
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(group_id)
        .fetch_all(pool)
        .await
}

pub async fn get_user(
    pool: &Pgsql,
    user_id: &uuid::Uuid
) -> Result<User, sqlx::Error> {
    let query = r#"
        SELECT
            id, group_id, name
        FROM
            users
        WHERE
            id = $1
    "#;

    sqlx::query_as<_, User>(query)
        .bind(user_id)
        .fetch_one(pool)
        .await
}

pub async fn create_user(
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

pub async fn delete_user(
    pool: &Pgsql,
    user_id: &uuid::Uuid
) -> Result<PgQueryResult, sqlx::Error> {
    let query = r#"
        DELETE FROM users
        WHERE
            id = &1
    "#;

    sqlx::query::<Postgres>(query)
        .bind(user_id)
        .await
}


pub async fn login(
    pool: &Pgsql,
    group_id: &uuid::Uuid,
    name: &str,
    psss: &str
) -> Result<User, sqlx::Error> {
    let query = r#"
        SELECT
            id, group_id, name
        FROM
            users
        WHERE
            name = $1 AND pass = $2
    "#;

    sqlx::query_as::<_, User>(query)
        .bind(name)
        .bind(pass)
        .fetch_one(pool)
        .await
}