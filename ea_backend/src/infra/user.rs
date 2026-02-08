use sqlx::{Pool, Postgres, types::uuid};


pub struct User {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
}

pub struct UserWithPass {
    pub id: uuid::Uuid,
    pub pass: String,
}


pub async fn create(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    pass: &str
) -> Result<uuid::Uuid, sqlx::Error> {
    let user_id = uuid::Uuid::new_v4();
    match sqlx::query!(
        r#"
        INSERT INTO users
            (id, name, email, pass)
        VALUES
            ($1, $2, $3, $4)
        "#,
        user_id,
        name,
        email,
        pass
    ).execute(pool).await {
        Ok(_) => Ok(user_id),
        Err(e) => {
            tracing::error!(error = %e, "failed to insert user");
            Err(e)
        }
    }
}

pub async fn get_users(pool: &Pool<Postgres>) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, name, email
        FROM
            users
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(users)
}

pub async fn find_by_email(pool: &Pool<Postgres>, email: &str) -> Result<UserWithPass, sqlx::Error> {
    let user = sqlx::query_as!(
        UserWithPass,
        r#"
        SELECT
            id, pass
        FROM
            users
        WHERE
            email = $1
        "#,
        email
    )
        .fetch_one(pool)
        .await?;

    Ok(user)
}

pub async fn find_by_id(pool: &Pool<Postgres>, id: uuid::Uuid) -> Result<User, sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT
            id, name, email
        FROM
            users
        WHERE
            id = $1
        "#,
        id
    )
        .fetch_one(pool)
        .await?;
    Ok(user)
}

pub async fn update(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
    name: &str,
    email: &str,
    pass: &str
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        UPDATE users
        SET name = $2, email = $3, pass = $4
        WHERE id = $1
        "#,
        id,
        name,
        email,
        pass
    )
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn delete(pool: &Pool<Postgres>, id: uuid::Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM
            users
        WHERE
            id = $1
        "#,
        id
    )
        .execute(pool)
        .await?;
    Ok(())
}
