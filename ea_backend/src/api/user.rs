use actix_web::{web, HttpResponse};
use serde::{Serialize, Deserialize};
use sqlx::types::uuid;
use crate::{service, core::AppState};


#[derive(Deserialize)]
pub struct LoginRequest {
    email: String,
    pass: String
}


#[derive(Serialize)]
pub struct LoginResponse {
    jwt: String,
    refresh: String
}


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
) -> HttpResponse {
    let body = payload.into_inner();
    let id = match service::user::register_user(
        &app_state.db_pool,
        &body.name,
        &body.email,
        &body.pass
    ).await {
        Ok(id) => id,
        Err(e) => {
            println!("user create Err: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string())
        }
    };
    println!("user created");
    HttpResponse::Created().json(CreateUserResponse{ id })
}


pub async fn get(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let user = match service::user::get_user(&app_state.db_pool, path.into_inner()).await {
        Ok(user) => user,
        Err(e) => {
            println!("Faild to get user: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    HttpResponse::Ok()
        .json(GetUserResponse{id: user.id, name: user.name, email: user.email})
}


pub async fn list(app_state: web::Data<AppState>) -> HttpResponse {
    let users = match service::user::get_users(&app_state.db_pool).await {
        Ok(users) => users,
        Err(e) => {
            println!("Faild to get user: {}", e.to_string());
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    let users: Vec<GetUserResponse> = users
        .into_iter()
        .map(|u| GetUserResponse{id: u.id, name: u.name, email: u.email} )
        .collect();

    let total = users.len();

    HttpResponse::Ok().json(ListUserResponse{ users, total })
}


pub async fn update(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CreateUserRequest>
) -> HttpResponse {
    let user_id = path.into_inner();
    match service::user::update_user(
        &app_state.db_pool,
        &user_id,
        &payload.name,
        &payload.email,
        &payload.pass
    ).await {
        Ok(_) => return HttpResponse::Ok().json(CreateUserResponse{id: user_id}),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    }
}

pub async fn delete(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
) -> HttpResponse {
    let user_id = path.into_inner();
    match service::user::delete_user(&app_state.db_pool, user_id).await{
        Ok(_) => return HttpResponse::NoContent().into(),
        Err(e) => {
            println!("user delete Err: {}", e.to_string());
            return HttpResponse::InternalServerError().into()
        }
    }
}
