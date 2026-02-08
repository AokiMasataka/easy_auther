use actix_web::{web, HttpResponse};
use serde::Serialize;
use crate::core::AppState;


#[derive(Serialize)]
pub struct JwkResponse {
    jwk: String
}


pub async fn jwk(
    app_state: web::Data<AppState>
) -> HttpResponse {
    let jwk = app_state.key_pair
        .public_key()
        .to_pem()
        .unwrap();
    HttpResponse::Ok().json(JwkResponse{ jwk })
}
