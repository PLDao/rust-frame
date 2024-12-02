use sea_orm::{ConnectOptions, Database, DbConn};
use std::{time::Duration};
use tracing::info;

pub async fn init_postgres_client(pgsql_url: &str) -> DbConn {
    let mut opt = ConnectOptions::new(pgsql_url);
    opt.max_connections(1000)
        .min_connections(5)
        .connect_timeout(Duration::from_secs(8))
        .idle_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_secs(8))
        .sqlx_logging(true);
    return Database::connect(opt).await.expect("Failed to connect to PostgreSQL");
}