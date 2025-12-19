use tracing::error;
use crate::feature::grpc::init_grpc;

mod docker;
mod grpc;

pub async fn init_feature() -> Result<(), Box<dyn std::error::Error>> {
    match init_grpc().await {
        Ok(_) => {}
        Err(err) => {
            error!("Grpc连接失败，请确认URL是否填写正常：error => {}", err);
            return Err(err);
        }
    };
    Ok(())
}