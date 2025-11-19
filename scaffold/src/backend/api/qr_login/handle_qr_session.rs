use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, DbErr, EntityTrait, QueryFilter, ColumnTrait, Set};
use tracing::info;
use crate::backend::models::qr_login_sessions;

pub async fn insert_qr_session(
    db: &DatabaseConnection,
    session_id: &str,
    ttl_seconds: i64,
) -> Result<qr_login_sessions::Model, DbErr> {
    let new_session = qr_login_sessions::ActiveModel {
        session_id: Set(session_id.to_string()),
        user_id: Set(None),
        status: Set("pending".to_string()),
        web_token: Set(None),
        app_token: Set(None),
        created_at: Set(Utc::now().naive_utc()),
        expires_at: Set(Utc::now().naive_utc() + Duration::seconds(ttl_seconds)),
        updated_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    };

    let inserted = new_session.insert(db).await?;
    info!("Inserted QR login session: {}", session_id);
    Ok(inserted)
}

pub async fn find_session_by_id(
    db: &DatabaseConnection,
    session_id: &str,
) -> Result<Option<qr_login_sessions::Model>, DbErr> {
    qr_login_sessions::Entity::find()
        .filter(qr_login_sessions::Column::SessionId.eq(session_id))
        .one(db)
        .await
}

pub async fn update_session_confirmed(
    db: &DatabaseConnection,
    session_id: &str,
    user_id: &str,
    web_token: &str,
    app_token: &str,
) -> Result<qr_login_sessions::Model, DbErr> {
    let session = find_session_by_id(db, session_id).await?
        .ok_or(DbErr::RecordNotFound("Session not found".to_string()))?;
    
    let mut session_active: qr_login_sessions::ActiveModel = session.into();
    session_active.status = Set("confirmed".to_string());
    session_active.user_id = Set(Some(user_id.to_string()));
    session_active.web_token = Set(Some(web_token.to_string()));
    session_active.app_token = Set(Some(app_token.to_string()));
    session_active.updated_at = Set(Utc::now().naive_utc());
    
    let updated = session_active.update(db).await?;
    info!("Updated QR login session {} to confirmed", session_id);
    Ok(updated)
}
