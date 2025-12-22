use sea_orm_migration::prelude::*;
use std::fs::create_dir_all;
use std::{fs, path};

#[async_std::main]
async fn main() {
    dotenvy::dotenv().ok();
    let path = path::Path::new("data/app.db");
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).unwrap();
        }
        fs::write(path, "").unwrap();
    }
    cli::run_cli(migration::Migrator).await;
}
