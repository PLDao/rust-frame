mod send_email;
mod send_phone;

use actix_web::{Scope, web};
use crate::backend::api::code::send_email::send_email_code;
use crate::backend::api::code::send_phone::send_phone_code;

pub fn code_scope() -> Scope {
    web::scope("/code")
        .route("/send-email", web::post().to(send_email_code))
        .route("/send-phone", web::post().to(send_phone_code))
}
