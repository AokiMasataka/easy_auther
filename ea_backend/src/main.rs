use actix_cors::Cors;
use actix_web::{web, http, middleware, App, HttpServer};

mod model;
mod api;
mod utils;

use api::{group, user};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let db_config = model::config::DataBaseConfig::new(
        std::env::var("PGSQL_HOST").expect("PGSQL_HOST must be set"),
        std::env::var("PGSQL_PORT").expect("PGSQL_PORT must be set"),
        std::env::var("PGSQL_USER").expect("PGSQL_USER must be set"),
        std::env::var("PGSQL_PASS").expect("PGSQL_PASS must be set"),
        std::env::var("PGSQL_DB").expect("PGSQL_DB must be set"),
    );

    let pool = db_config.connection().await;
    let port = std::env::var("BACKEND_PORT")
        .expect("BACKEND_PORT must be set")
        .parse::<u16>()
        .expect("Plz input port");
    println!("serve on http://localhost:{}", port);
    
    let private_key = utils::jwt::create_private_key();

    HttpServer::new( move || {
        //let cors = Cors::default()
        //    .allowed_origin("http://localhost:3000/")
        //    .allowed_origin("")
        //    .allowed_methods(vec!["GET", "POST", "DELETE"])
        //    .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        //    .allowed_header(http::header::CONTENT_TYPE)
        //    .max_age(3600);
        let cors = Cors::permissive();

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(private_key.clone()))
            .route("/login", web::post().to(group::login))
            .route("/refresh", web::post().to(group::refresh))
            .route("/group", web::post().to(group::create))
            .service(
                web::scope("/{group_id}")
                    .wrap(middleware::from_fn(utils::authorize_middleware))
                        .route("/", web::put().to(group::update))
                        .route("/", web::delete().to(group::delete))
                        .route("/users", web::get().to(group::get_users))
                        .route("/user", web::post().to(user::create))
                        .route("/user/{user_id}", web::delete().to(user::delete))
                        .route("/login", web::post().to(user::login))
                        .route("/refresh", web::post().to(user::refresh))
                        .route("/verify", web::post().to(user::verify))
            )
    })
    .bind(("0.0.0.0", port))?
    .run()
    .await
}
