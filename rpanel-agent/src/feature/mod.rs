use tracing::error;
use crate::feature::grpc::init_grpc;

mod docker;
mod grpc;

pub async fn init_feature() {
    tokio::spawn(init_grpc());
}