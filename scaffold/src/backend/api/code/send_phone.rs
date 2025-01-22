use actix_web::{HttpResponse, web};
use sea_orm::DatabaseConnection;
use serde::Deserialize;
use tracing::info;
use crate::backend::AppState;

#[derive(Deserialize, Debug)]
pub struct PhoneRequest {
    pub phone: String,
}

pub async fn send_phone_code(
    state: web::Data<AppState>,
    request: web::Json<PhoneRequest>,
) -> HttpResponse {
    info!( "Received email request: {:?}", request);
    println!("Received email request: {:?}", request);
    info!("Received email request: {:?}", request);
    HttpResponse::Ok().body("phone code sent successfully")
}