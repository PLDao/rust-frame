use sea_orm::DbConn;

pub mod app_router;



#[derive(Clone)]
pub struct AppState {
    pub pg_client: DbConn
}