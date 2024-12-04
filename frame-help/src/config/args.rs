use clap::{Parser, Subcommand};
#[derive(Parser)]
#[command(name = "frame-help")]
#[command(about = "An application to manage contracts", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub(crate) command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Starts the deployment process for all contracts
    NewJwtKey,
}