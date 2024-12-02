use tracing::info;
use tokio::signal;
use crate::config::arg::Args;
use crate::config::log::init_tracing;
use crate::config::pg::init_postgres_client;
use clap::Parser;
use crate::backend::app_router::run_backend_server;

mod config;
mod backend;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    init_tracing();
    info!("🔧 Initializing the Actix-Web server...");

    let args = Args::parse();

    let pg_client = init_postgres_client(&args.pgsql_url).await;
    info!("✅ Successfully connected to PostgreSQL database.");

    let server = run_backend_server(pg_client.clone(), args.backend_port);

    tokio::select! {
        _ = server => {
            info!("🚀 Server has shut down.");
        }
        _ = signal::ctrl_c() => {
            info!("🔴 Received shutdown signal.");
        }
    }

    // 在这里执行清理逻辑
    info!("🧹 Cleaning up resources...");
    drop(pg_client); // 手动释放数据库连接池

    info!("✅ Cleanup completed. Exiting.");
    Ok(())
}