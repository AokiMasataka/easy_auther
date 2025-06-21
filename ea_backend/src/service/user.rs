use sqlx::{Pool, Postgres, types::uuid};
use crate::infra;

type User = infra::user::User;


pub async fn login(
    pool: &Pool<Postgres>,
    email: &str,
    pass: &str
) -> Result<Option<User>, sqlx::Error> {
    let user = infra::user::find_by_email(&pool, &email).await?;
    
    if user.pass == pass {
        return Ok(
            Some(
                User{
                    id: user.id,
                    name: user.name,
                    email: user.email
                }
            )
        );
    } else {
        return Ok(None);
    }
}


pub async fn register_user(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    pass: &str
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = infra::user::create(&pool, name, email, pass).await?;
    Ok(id)
}


pub async fn get_users(
    pool: &Pool<Postgres>
) -> Result<Vec<User>, sqlx::Error> {
    let users = infra::user::get_users(&pool).await?;
    Ok(users)
}

pub async fn get_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid
) -> Result<User, sqlx::Error> {
    let user = infra::user::find_by_id(&pool, id).await?;
    Ok(user)
}


pub async fn update_user(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
    name: &str,
    email: &str,
    pass: &str
) -> Result<(), sqlx::Error> {
    infra::user::update(&pool, id, name, email, pass).await?;
    Ok(())
}


pub async fn delete_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid
) -> Result<(), sqlx::Error> {
    infra::user::delete(&pool, id).await?;
    Ok(())
}
