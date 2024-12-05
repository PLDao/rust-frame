use crate::backend::models::api_users;
use crate::backend::models::api_users::{ActiveModel, Model};
use crate::backend::utils::hash::{hash_password, verify_password};
use sea_orm::{entity::*, query::*, DatabaseConnection, DbErr};

pub struct UserService;

impl UserService {
    /// 根据 user_id 查找用户
    pub async fn find_by_userid(
        db: &DatabaseConnection,
        user_id: &str,
    ) -> Result<Model, DbErr> {
        api_users::Entity::find()
            .filter(api_users::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("User with id {} not found", user_id)))
    }

    /// 根据用户名和密码验证用户
    pub async fn find_by_username_and_password(
        db: &DatabaseConnection,
        username: &str,
        password: &str,
    ) -> Result<Model, DbErr> {
        let user = api_users::Entity::find()
            .filter(api_users::Column::Username.eq(username))
            .one(db)
            .await?
            .ok_or_else(|| DbErr::RecordNotFound(format!("User with username {} not found", username)))?;

        if !verify_password(password, &user.password_hash) {
            return Err(DbErr::Custom("Invalid password".to_string()));
        }

        if !user.verified {
            return Err(DbErr::Custom("User not verified, please await verification".to_string()));
        }

        Ok(user)
    }

    /// 插入新用户
    pub async fn insert_user(
        db: &DatabaseConnection,
        user_id: &str,
        username: &str,
        password_hash: &str,
    ) -> Result<api_users::Model, DbErr> {
        let new_user = api_users::ActiveModel {
            user_id: Set(user_id.to_string()),
            username: Set(username.to_string()),
            password_hash: Set(password_hash.to_string()),
            ..Default::default()
        };

        new_user.insert(db).await
    }

    /// 注册用户
    pub async fn register_user(
        db: &DatabaseConnection,
        user_id: &str,
        username: &str,
        password: &str,
    ) -> Result<Model, DbErr> {
        if api_users::Entity::find()
            .filter(api_users::Column::UserId.eq(user_id))
            .one(db)
            .await?
            .is_some()
        {
            return Err(DbErr::Custom(format!("User ID {} is already taken", user_id)));
        }

        if api_users::Entity::find()
            .filter(api_users::Column::Username.eq(username))
            .one(db)
            .await?
            .is_some()
        {
            return Err(DbErr::Custom(format!("Username {} is already taken", username)));
        }

        let password_hash = hash_password(password).map_err(|e| DbErr::Custom(e.to_string()))?;

        Self::insert_user(db, user_id, username, &password_hash).await
    }
}