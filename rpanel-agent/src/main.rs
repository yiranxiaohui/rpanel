use crate::config::init_config;
use crate::feature::init_feature;

mod feature;
mod config;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();
    init_config()?;
    init_feature().await?;
    // 阻塞主线程，直到 Ctrl+C
    tokio::signal::ctrl_c().await?;
    tracing::info!("shutdown signal received");
    Ok(())
}