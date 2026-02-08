use sqlx::{Pool, Postgres, types::uuid};
use crate::{infra, core::exceptions::AppError};

type User = infra::user::User;


pub async fn register_user(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    pass: &str
) -> Result<uuid::Uuid, AppError> {
    let id = infra::user::create(&pool, name, email, pass)
        .await
        .map_err(AppError::from)?;
    Ok(id)
}


pub async fn get_users(
    pool: &Pool<Postgres>
) -> Result<Vec<User>, AppError> {
    let users = infra::user::get_users(&pool)
        .await
        .map_err(AppError::from)?;
    Ok(users)
}

pub async fn get_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid
) -> Result<User, AppError> {
    let user = infra::user::find_by_id(&pool, id)
        .await
        .map_err(AppError::from)?;
    Ok(user)
}


pub async fn update_user(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
    name: &str,
    email: &str,
    pass: &str
) -> Result<(), AppError> {
    infra::user::update(&pool, id, name, email, pass)
        .await
        .map_err(AppError::from)?;
    Ok(())
}


pub async fn delete_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid
) -> Result<(), AppError> {
    infra::user::delete(&pool, id)
        .await
        .map_err(AppError::from)?;
    Ok(())
}
