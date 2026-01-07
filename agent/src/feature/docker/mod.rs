use std::error::Error;
use std::sync::OnceLock;
use bollard::{Docker, API_DEFAULT_VERSION};
use crate::config::get_config;

pub mod container;
pub mod image;

pub static DOCKER: OnceLock<Docker> = OnceLock::new();

pub fn get_docker() -> &'static Docker {
    let config = get_config();
    let url = &config.docker;

    DOCKER.get_or_init(|| {
        docker_from_endpoint(url.as_str()).expect("failed to connect docker")
    })
}

fn docker_from_endpoint(endpoint: &str) -> Result<Docker, Box<dyn Error>> {
    let timeout = 120;
    let version = API_DEFAULT_VERSION;
    // Unix socket（绝对路径）
    if endpoint.starts_with('/') {
        return Ok(Docker::connect_with_local(
            endpoint,
            120,
            API_DEFAULT_VERSION,
        )?);
    }
    // unix://
    if let Some(path) = endpoint.strip_prefix("unix://") {
        return Ok(Docker::connect_with_socket(
            path,
            timeout,
            version,
        )?);
    }

    // tcp:// -> http://
    let endpoint = if endpoint.starts_with("tcp://") {
        endpoint.replacen("tcp://", "http://", 1)
    } else {
        endpoint.to_string()
    };

    // http / https
    if endpoint.starts_with("http://") || endpoint.starts_with("https://") {
        return Ok(Docker::connect_with_http(
            &endpoint,
            timeout,
            version,
        )?);
    }

    Err(format!("Unsupported docker endpoint: {}", endpoint).into())
}