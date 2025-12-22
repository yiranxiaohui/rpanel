mod feature;
mod config;

use tracing_subscriber::{fmt, EnvFilter};
use crate::config::init_config;
use crate::feature::init_feature;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    fmt()
        .with_env_filter(filter)
        .init();
    init_config()?;
    init_feature().await;
    // 阻塞主线程，直到 Ctrl+C
    tokio::signal::ctrl_c().await?;
    Ok(())
}