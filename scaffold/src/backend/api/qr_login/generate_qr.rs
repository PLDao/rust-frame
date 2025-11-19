use actix_web::{web, HttpResponse};
use serde::Deserialize;
use uuid::Uuid;
use tracing::info;
use crate::backend::AppState;
use crate::backend::api::qr_login::handle_qr_session::insert_qr_session;

#[derive(Deserialize, Debug)]
pub struct GenerateQrRequest {
    pub client_info: Option<String>,
}

pub async fn generate_qr_code(
    state: web::Data<AppState>,
    request: web::Json<GenerateQrRequest>,
) -> HttpResponse {
    info!("Received generate QR code request: {:?}", request);
    
    let session_id = Uuid::new_v4().to_string();
    let ttl_seconds = 300; // 5 minutes
    
    // 创建登录会话
    if let Err(e) = insert_qr_session(&state.pg_client, &session_id, ttl_seconds).await {
        return HttpResponse::InternalServerError().body(format!("Failed to create QR session: {}", e));
    }
    
    // 构造二维码数据
    let qr_data = format!(
        r#"{{"session_id":"{}","action":"login","expires_at":{}}}"#,
        session_id,
        chrono::Utc::now().timestamp() + ttl_seconds
    );
    
    // 返回响应
    let response = format!(
        r#"{{"session_id":"{}","qr_data":"{}","expires_in":{}}}"#,
        session_id,
        qr_data.replace("\"", "\\\""),
        ttl_seconds
    );
    
    info!("Generated QR code for session: {}", session_id);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}
