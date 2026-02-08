use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::types::uuid;
use crate::{core::{AppState, exceptions::AppError}, service};


#[derive(Deserialize)]
pub struct CreateUserRequest {
    name: String,
    email: String,
    pass: String
}

#[derive(Serialize)]
pub struct CreateUserResponse {
    id: uuid::Uuid
}


#[derive(Serialize)]
pub struct GetUserResponse {
    id: uuid::Uuid,
    name: String,
    email: String
}


#[derive(Serialize)]
pub struct ListUserResponse {
    users: Vec<GetUserResponse>,
    total: usize
}


pub async fn create(
    app_state: web::Data<AppState>,
    payload: web::Json<CreateUserRequest>
) -> Result<HttpResponse,  AppError> {
    let body = payload.into_inner();
    let id = service::user::register_user(
        &app_state.db_pool,
        &body.name,
        &body.email,
        &body.pass
    )
        .await?;
    Ok(
        HttpResponse::Created()
            .json(CreateUserResponse{ id })
    )
}


pub async fn get(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> Result<HttpResponse,  AppError> {
    let user = service::user::get_user(&app_state.db_pool, path.into_inner())
        .await?;

    Ok(
        HttpResponse::Ok()
            .json(GetUserResponse{id: user.id, name: user.name, email: user.email})
    )
}


pub async fn list(app_state: web::Data<AppState>) -> Result<HttpResponse,  AppError> {
    let users = service::user::get_users(&app_state.db_pool).await?;

    let users: Vec<GetUserResponse> = users
        .into_iter()
        .map(|u| GetUserResponse{id: u.id, name: u.name, email: u.email} )
        .collect();

    let total = users.len();

    Ok(HttpResponse::Ok().json(ListUserResponse{ users, total }))
}


pub async fn update(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CreateUserRequest>
) -> Result<HttpResponse,  AppError> {
    let user_id = path.into_inner();
    
    service::user::update_user(
        &app_state.db_pool,
        &user_id,
        &payload.name,
        &payload.email,
        &payload.pass
    ).await?;

    Ok(HttpResponse::Ok().json(CreateUserResponse{id: user_id}))
}

pub async fn delete(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> Result<HttpResponse,  AppError> {
    let user_id = path.into_inner();
    service::user::delete_user(&app_state.db_pool, user_id).await?;

    Ok(HttpResponse::NoContent().finish())
}
