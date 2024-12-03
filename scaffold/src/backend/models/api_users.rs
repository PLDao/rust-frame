//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.1

use super::sea_orm_active_enums::RoleType;
use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "api__users")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i64,
    #[sea_orm(column_type = "Text", unique)]
    pub user_id: String,
    #[sea_orm(column_type = "Text", unique)]
    pub username: String,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub email: Option<String>,
    #[sea_orm(column_type = "Text", nullable, unique)]
    pub phone: Option<String>,
    #[sea_orm(column_type = "Text")]
    pub password_hash: String,
    pub verified: bool,
    pub role: Option<RoleType>,
    pub created_at: DateTime,
    pub updated_at: Option<DateTime>,
    pub last_login_at: Option<DateTime>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::api_keys::Entity")]
    ApiKeys,
}

impl Related<super::api_keys::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::ApiKeys.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
