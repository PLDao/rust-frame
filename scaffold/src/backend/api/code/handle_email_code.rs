use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use sea_orm::ActiveValue::Set;
use tracing::info;
use crate::backend::models::email_verifications;

pub async fn insert_email_code(
    db: &DatabaseConnection,
    user_id: &str,
    email: &str,
    code: &str,
    ttl_seconds: i64,
) -> Result<email_verifications::Model, DbErr> {
    let new_code = email_verifications::ActiveModel {
        user_id: Set(user_id.to_string()),  // 修正：增加 user_id
        email: Set(email.to_string()),
        code: Set(code.to_string()),
        is_used: Set(Some(false)),  // 修正：默认 is_used 为 false
        created_at: Set(Utc::now().naive_utc()),
        expires_at: Set(Utc::now().naive_utc() + Duration::seconds(ttl_seconds)),
        ..Default::default() // 避免遗漏其他字段
    };

    let inserted = new_code.insert(db).await?;
    info!("Inserted email verification code for {}", email);
    Ok(inserted)
}
