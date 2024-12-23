use dotenv;
use actix_cors::Cors;
use actix_web::{web, http, App, HttpServer};

mod model;
mod group;
mod user;
mod middleweres;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::from_path("../.env").ok();

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
    
    let private_key = group::token::create_private_key();

    HttpServer::new( move || {
        let cors = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "DELETE"])
        .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
        .allowed_header(http::header::CONTENT_TYPE)
        .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(private_key.clone()))
            .route("/login", web::post().to(group::controller::login_group))
            .route("/refresh", web::post().to(group::controller::refresh_token))
            .route("/group", web::post().to(group::controller::create_group))
            .route("/{group_id}", web::delete().to(group::controller::delete_group))
            .service(
                web::scope("/{group_id}")
                    .route("/user", web::post().to(user::controller::create_user))
            )
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
