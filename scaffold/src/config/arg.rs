use clap::Parser;

#[derive(Parser, Clone, Debug)]
pub struct Args {
    /// PostgreSQL database URL
    #[arg(short, long, default_value = "postgres://postgres:postgres@localhost:5432/postgres")]
    pub(crate) pgsql_url: String,
    /// Server port
    #[arg(short, long, default_value_t = 3000)]
    pub(crate) backend_port: u16,
}
