use tracing::{error, info, warn};
use crate::feature::database::init_database;
use crate::feature::grpc::init_grpc;

pub mod grpc;
pub mod docker;
mod database;

pub async fn init_feature() {
    tokio::spawn(async {
        info!("grpc task entered");
        init_grpc().await;
        info!("grpc task exited");
        println!("grpc task exited");
    });
    tokio::spawn(init_database());
}