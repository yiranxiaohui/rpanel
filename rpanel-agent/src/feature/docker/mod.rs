use std::sync::OnceLock;
use bollard::{Docker, API_DEFAULT_VERSION};

pub mod container;
pub mod image;

pub static DOCKER: OnceLock<Docker> = OnceLock::new();

fn get_docker() -> &'static Docker {
    DOCKER.get_or_init(|| {
        Docker::connect_with_http(
            "http://10.0.12.1:2375",
            120,
            API_DEFAULT_VERSION,
        )
        .expect("failed to connect docker")
    })
}