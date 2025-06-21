use sqlx::{Pool, Postgres, types::uuid};
use crate::infra;

type ManageUser = infra::manager::ManageUser;


pub async fn login(
    pool: &Pool<Postgres>,
    email: &str,
    pass: &str
) -> Result<Option<ManageUser>, sqlx::Error> {
    let user = infra::manager::find_by_email(&pool, &email).await?;

    if user.pass == pass {
        return Ok(Some(ManageUser{id: user.id, name: user.name, email: user.email}));
    } else {
        return Ok(None);
    }
}


pub async fn register_user(
    pool: &Pool<Postgres>,
    name: &str,
    email: &str,
    pass: &str,
) -> Result<uuid::Uuid, sqlx::Error> {
    let id = infra::manager::create(
        &pool, name, email, pass
    ).await?;
    Ok(id)
}

pub async fn get_managers(
    pool: &Pool<Postgres>,
) -> Result<Vec<ManageUser>, sqlx::Error> {
    let users = infra::manager::get_managers(&pool).await?;
    Ok(users)
}


pub async fn get_manager(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid
) -> Result<ManageUser, sqlx::Error> {
    let manager = infra::manager::find_by_id(&pool, id).await?;
    Ok(manager)
}


pub async fn update_manager(
    pool: &Pool<Postgres>,
    id: &uuid::Uuid,
    name: &str,
    email: &str,
    pass: &str
) -> Result<(), sqlx::Error> {
    infra::manager::update(&pool, id, name, email, pass).await?;
    Ok(())
}


pub async fn delete_user(
    pool: &Pool<Postgres>,
    id: uuid::Uuid,
) -> Result<(), sqlx::Error> {
    infra::manager::delete(&pool, id).await?;
    Ok(())
}
