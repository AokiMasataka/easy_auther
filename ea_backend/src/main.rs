use std::process::exit;

use dotenvy::dotenv;
use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};
use tracing_subscriber::EnvFilter;

mod api;
mod service;
mod infra;
mod core;

use crate::api::middlewares::{access_log, validate_jwt, validate_secret};


fn cors_config() -> Cors {
    Cors::default()
        .allowed_origin("http://localhost:3001")
        .allowed_origin("http://localhost:3000")
        //.allow_any_origin()
        .allowed_methods(vec!["GET", "POST", "DELETE", "OPTIONS"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .supports_credentials()
        .max_age(3600)
}


async fn init_manager(state: &core::AppState) {
    match infra::manager::find_by_name(&state.db_pool, "root").await{
        Ok(_) => tracing::info!("exist root user"),
        Err(_) => {
            tracing::info!(email=&state.config.root_email, "creating root user");
            service::manager::register_user(
                &state.db_pool,
                "root",
                &state.config.root_email,
                &state.config.root_pass
            )
                .await
                .unwrap();
        }
    };
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .json()
        .init();

    let state = match core::AppState::from_env().await {
        Ok(state) => state,
        Err(error) => {
            panic!("AppState Error: {}", error);
        }
    };
    
    let app_port = state.config.port;
    match sqlx::migrate!().run(&state.db_pool).await {
        Ok(_) => tracing::info!("DB migrated"),
        Err(e) => {
            tracing::error!(error=e.to_string(), "migrate error");
            exit(1)
        }
    };

    init_manager(&state).await;

    HttpServer::new( move || {
        App::new()
            .wrap(actix_web::middleware::from_fn(access_log))
            .wrap(cors_config())
            .app_data(web::Data::new(state.clone()))
            .route("/jwk", web::get().to(api::jwk::jwk))
            .service(
                web::scope("/manage")
                .route("/login", web::post().to(api::manager::login))
                .service(
                    web::scope("/managers")
                    .wrap(actix_web::middleware::from_fn(validate_jwt))
                    .route("", web::post().to(api::manager::create))
                    .route("", web::get().to(api::manager::list))
                    .route("/{user_id}", web::get().to(api::manager::get))
                    .route("/{user_id}", web::put().to(api::manager::update))
                    .route("/{user_id}", web::delete().to(api::manager::delete))
                )
                .service(
                    web::scope("/users")
                    .wrap(actix_web::middleware::from_fn(validate_jwt))
                    .route("", web::post().to(api::user::create))
                    .route("", web::get().to(api::user::list))
                    .route("/{user_id}", web::get().to(api::user::get))
                    .route("/{user_id}", web::put().to(api::user::update))
                    .route("/{user_id}", web::delete().to(api::user::delete))
                )
            )
            .route("/authorize", web::post().to(api::auth::auth))
            .route("/token", web::post().to(api::auth::token))
            .route("/refresh", web::post().to(api::auth::refresh_token))
            .route("/logout", web::post().to(api::auth::logout))
            .service(
                web::scope("/")
                .wrap(actix_web::middleware::from_fn(validate_secret))
                .route("/users", web::post().to(api::user::create))
                .route("/users", web::get().to(api::user::list))
                .route("/users/{user_id}", web::get().to(api::user::get))
                .route("/users/{user_id}", web::put().to(api::user::update))
                .route("/users/{user_id}", web::delete().to(api::user::delete))
            )
    })
        .workers(1)
        .bind(("0.0.0.0", app_port))?
        .run()
        .await
}
