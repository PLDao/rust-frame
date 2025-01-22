use sea_orm::DbConn;

mod models;
pub mod app_router;
mod middleware;
mod handlers;
mod utils;
mod api;

#[derive(Clone)]
pub struct AppState {
    pub pg_client: DbConn
}