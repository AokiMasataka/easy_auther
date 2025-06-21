use sqlx::{Pool, Postgres, types::uuid};

pub struct ManageUser {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String
}


pub struct ManageUserWithPass {
    pub id: uuid::Uuid,
    pub name: String,
    pub email: String,
    pub pass: String,
}


pub async fn create(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    pass: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = uuid::Uuid::new_v4();
    sqlx::query!(
        r#"
        INSERT INTO managers
            (id, name, email, pass)
        VALUES
            ($1, $2, $3, $4)
        "#,
        id,
        name,
        email,
        pass,
    )
        .execute(pool)
        .await?;
    Ok(id)
}


pub async fn find_by_email(
    pool: &Pool<Postgres>, email: &str
) -> Result<ManageUserWithPass, sqlx::Error> {
    let user = sqlx::query_as!(
        ManageUserWithPass,
        r#"
        SELECT
            id, name, email, pass
        FROM
            managers
        WHERE
            email = $1
        "#,
        email
    )
        .fetch_one(pool)
        .await?;
    Ok(user)
}


pub async fn find_by_name(
    pool: &Pool<Postgres>, name: &str
) -> Result<ManageUserWithPass, sqlx::Error> {
    let user = sqlx::query_as!(
        ManageUserWithPass,
        r#"
        SELECT
            id, name, email, pass
        FROM
            managers
        WHERE
            name = $1
        "#,
        name
    )
        .fetch_one(pool)
        .await?;
    Ok(user)
}


pub async fn find_by_id(
    pool: &Pool<Postgres>, id: &uuid::Uuid
) -> Result<ManageUser, sqlx::Error> {
    let user = sqlx::query_as!(
        ManageUser,
        r#"
        SELECT
            id, name, email
        FROM
            managers
        WHERE
            id = $1
        "#,
        id
    )
        .fetch_one(pool)
        .await?;
    Ok(user)
}


pub async fn get_managers(
    pool: &Pool<Postgres>,
) -> Result<Vec<ManageUser>, sqlx::Error> {
    let managers = sqlx::query_as!(
        ManageUser,
        r#"
        SELECT
            id, name, email
        FROM
            managers
        "#
    )
        .fetch_all(pool)
        .await?;

    Ok(managers)
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
        UPDATE managers
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


pub async fn delete(
    pool: &Pool<Postgres>,
    id: uuid::Uuid
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM
            managers
        WHERE
            id = $1
        "#,
        id
    )
        .execute(pool)
        .await?;
    Ok(())
}
