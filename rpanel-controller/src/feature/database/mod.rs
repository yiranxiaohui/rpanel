use sea_orm::DatabaseConnection;
use crate::feature::database::sqlite::{get_sqlite_database_connection, init_sqlite};

mod sqlite;

pub async fn init_database() {
    init_sqlite().await;
}

pub async fn get_database() -> DatabaseConnection {
    get_sqlite_database_connection().await
}