use sea_orm::DbConn;

pub mod models;
pub mod app_router;
mod middleware;
mod utils;
mod api;

#[derive(Clone)]
pub struct AppState {
    pub pg_client: DbConn
}