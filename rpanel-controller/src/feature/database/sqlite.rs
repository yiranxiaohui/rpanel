use std::fs::create_dir_all;
use std::{fs, path};
use sea_orm::{Database, DatabaseConnection};
use sea_orm_migration::MigratorTrait;
use migration::Migrator;

pub async fn init_sqlite() -> DatabaseConnection {
    let path = path::Path::new("data/app.db");
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_dir_all(parent).unwrap();
        }
        fs::write(path, "").unwrap();
    }
    let db = Database::connect("sqlite://data/app.db")
        .await
        .expect("failed to connect sqlite");

    // 自动执行所有未执行的 migration
    Migrator::up(&db, None)
        .await
        .expect("migration failed");

    db
}

pub async fn get_sqlite_database_connection() -> DatabaseConnection {
    Database::connect("sqlite://data/app.db")
        .await
        .expect("failed to connect sqlite")
}