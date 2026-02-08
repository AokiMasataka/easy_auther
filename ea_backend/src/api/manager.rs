use actix_web::{HttpResponse, web};
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
pub struct CreateManagerRequest {
    name: String,
    email: String,
    pass: String
}


#[derive(Serialize)]
pub struct CreateManagerResponse {
    id: uuid::Uuid
}


#[derive(Serialize)]
pub struct GetManagerResponse {
    id: uuid::Uuid,
    name: String,
    email: String
}


#[derive(Serialize)]
pub struct ListManagerResponse {
    users: Vec<GetManagerResponse>,
    total: usize
}


pub async fn login(
    app_state: web::Data<AppState>,
    payload: web::Json<LoginRequest>
) -> HttpResponse {
    tracing::info!(email=&payload.email, "login manager");
    let user = match service::manager::login(
        &app_state.db_pool, &payload.email, &payload.pass
    ).await {
        Ok(user) => user,
        Err(_) => return HttpResponse::Unauthorized().body("")
    };

    let user = match user {
        Some(user) => user,
        None => return HttpResponse::Unauthorized().body("")
    };

    let response = LoginResponse{
        jwt: service::signature::sign(&app_state.key_pair, user.id).unwrap(),
        refresh: service::signature::sign(&app_state.key_pair, user.id).unwrap()
    };

    HttpResponse::Ok().json(response)
}


pub async fn create(
    app_state: web::Data<AppState>,
    payload: web::Json<CreateManagerRequest>
) -> HttpResponse {
    let body = payload.into_inner();
    let id = match service::manager::register_user(
        &app_state.db_pool,
        &body.name,
        &body.email,
        &body.pass
    ).await {
        Ok(id) => id,
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    };
    tracing::info!(user_id = id.to_string(), "manager created");
    HttpResponse::Created().body(id.to_string())
}


pub async fn get(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>
) -> HttpResponse {
    let user_id = path.into_inner();
    tracing::info!(user_id = user_id.to_string(), "get manager");
    let user = match service::manager::get_manager(&app_state.db_pool, &user_id).await {
        Ok(user) => user,
        Err(e) => {
            tracing::error!(error = e.to_string(), "Faild to get manager");
            return HttpResponse::InternalServerError().body(e.to_string());
        }
    };

    HttpResponse::Ok()
        .json(GetManagerResponse{id: user.id, name: user.name, email: user.email})
}

pub async fn list(
    app_state: web::Data<AppState>,
) -> HttpResponse {
    tracing::info!("list menager");
    let managers = match service::manager::get_managers(&app_state.db_pool).await {
        Ok(managers) => managers,
        Err(e) => {
            tracing::error!(error = e.to_string(), "Faild to list menager");
            return HttpResponse::InternalServerError().body(e.to_string())
        }
    };

    let users: Vec<GetManagerResponse> = managers
        .into_iter()
        .map(|m| GetManagerResponse{id: m.id, name: m.name, email: m.email})
        .collect();

    let total = users.len();
        
    HttpResponse::Ok().json(ListManagerResponse{users, total})
}


pub async fn update(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>,
    payload: web::Json<CreateManagerRequest>
) -> HttpResponse {
    let user_id = path.into_inner();
    tracing::info!(user_id = user_id.to_string(), "update manager");
    match service::manager::update_manager(
        &app_state.db_pool,
        &user_id,
        &payload.name,
        &payload.email,
        &payload.pass
    ).await {
        Ok(_) => return HttpResponse::Ok().json(CreateManagerResponse{id: user_id}),
        Err(e) => return HttpResponse::InternalServerError().body(e.to_string())
    }
}


pub async fn delete(
    app_state: web::Data<AppState>,
    path: web::Path<uuid::Uuid>
) -> HttpResponse {
    let user_id = path.into_inner();
    tracing::info!(user_id = user_id.to_string(), "delete manager");
    let response = match service::manager::delete_user(&app_state.db_pool, user_id).await {
        Ok(_) => HttpResponse::NoContent().body(""),
        Err(e) => {
            tracing::error!(error = e.to_string(), "Faild to delete manager");
            HttpResponse::InternalServerError().body("")
        }
    };
    response
}
