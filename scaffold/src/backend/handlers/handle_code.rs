use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr};
use sea_orm::ActiveValue::Set;
use tracing::info;
use crate::backend::models::email_verification_codes;

pub async fn insert_email_code(
    db: &DatabaseConnection,
    email: &str,
    code: &str,
    ttl_seconds: i64,
) -> Result<email_verification_codes::Model, DbErr> {
    let new_code = email_verification_codes::ActiveModel {
        email: Set(email.to_string()),
        code: Set(code.to_string()),
        created_at: Set(Utc::now().naive_utc()),
        expires_at: Set(Utc::now().naive_utc() + Duration::seconds(ttl_seconds)),
    };
    new_code.insert(db).await
}
