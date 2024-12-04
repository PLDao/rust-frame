use clap::Parser;
use tracing::info;
use crate::config::args::{Cli, Commands};
use crate::config::log::init_tracing;
use crate::tools::jwt::new_keys;

mod config;
mod tools;

fn main() {
    init_tracing();
    info!("🔧 Initializing the Actix-Web server...");

    let cli = Cli::parse();

    match cli.command {
        Commands::NewJwtKey => {
            info!("🚀 Starting the new jwt key...");
            new_keys();
            info!("✅ Finished the new jwt key.");
        }
    }
}
