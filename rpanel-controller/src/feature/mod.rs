use crate::feature::axum::init_axum;
use crate::feature::database::init_database;
use crate::feature::grpc::init_grpc;

pub mod grpc;
pub mod docker;
pub mod event;
mod database;
mod axum;

pub async fn init_feature() {
    tokio::spawn(init_axum());
    tokio::spawn(init_grpc());
    tokio::spawn(init_database());
}