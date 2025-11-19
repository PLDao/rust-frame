use crate::backend::api::qr_login::handle_qr_session::insert_qr_session;
use crate::backend::AppState;
use actix_web::{web, HttpResponse};
use base64::{engine::general_purpose, Engine as _};
use image::Luma;
use qrcode::QrCode;
use serde::Deserialize;
use tracing::info;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct GenerateQrRequest {
    pub client_info: Option<String>,
}

// 生成二维码图片并返回base64编码
fn generate_qr_image(data: &str) -> Result<String, String> {
    // 生成二维码
    let code =
        QrCode::new(data.as_bytes()).map_err(|e| format!("Failed to generate QR code: {}", e))?;

    // 渲染为图像
    let image = code
        .render::<Luma<u8>>()
        .min_dimensions(300, 300)
        .max_dimensions(300, 300)
        .build();

    // 转换为PNG字节
    let mut png_bytes: Vec<u8> = Vec::new();
    image::DynamicImage::ImageLuma8(image)
        .write_to(
            &mut std::io::Cursor::new(&mut png_bytes),
            image::ImageFormat::Png,
        )
        .map_err(|e| format!("Failed to encode PNG: {}", e))?;

    // Base64编码
    let base64_image = general_purpose::STANDARD.encode(&png_bytes);
    Ok(format!("data:image/png;base64,{}", base64_image))
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
        return HttpResponse::InternalServerError()
            .body(format!("Failed to create QR session: {}", e));
    }

    // 构造二维码数据
    let qr_data = format!(
        r#"{{"session_id":"{}","action":"login","expires_at":{}}}"#,
        session_id,
        chrono::Utc::now().timestamp() + ttl_seconds
    );

    // 生成二维码图片
    let qr_image = match generate_qr_image(&qr_data) {
        Ok(img) => img,
        Err(e) => {
            return HttpResponse::InternalServerError()
                .body(format!("Failed to generate QR image: {}", e));
        }
    };

    // 返回响应（包含base64图片）
    let response = format!(
        r#"{{"session_id":"{}","qr_image":"{}","qr_data":"{}","expires_in":{}}}"#,
        session_id,
        qr_image,
        qr_data.replace("\"", "\\\""),
        ttl_seconds
    );

    info!("Generated QR code image for session: {}", session_id);
    HttpResponse::Ok()
        .content_type("application/json")
        .body(response)
}
