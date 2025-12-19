use clap::Parser;
use crate::config::args::{load_or_init_config, Args};

mod feature;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    let args = Args::parse();
    load_or_init_config(&args)?;
    Ok(())
}