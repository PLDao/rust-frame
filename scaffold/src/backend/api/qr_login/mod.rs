mod generate_qr;
mod confirm_login;
mod check_status;
mod handle_qr_session;

use actix_web::{Scope, web};
use crate::backend::api::qr_login::generate_qr::generate_qr_code;
use crate::backend::api::qr_login::confirm_login::confirm_login;
use crate::backend::api::qr_login::check_status::check_login_status;

pub fn qr_login_scope() -> Scope {
    web::scope("/qr-login")
        .route("/generate", web::post().to(generate_qr_code))
        .route("/status/{session_id}", web::get().to(check_login_status))
        .route("/confirm", web::post().to(confirm_login))
}
