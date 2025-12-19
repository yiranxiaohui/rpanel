pub mod args;

use std::io::Error;
use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;
use uuid::Uuid;
use crate::config::args::{load_or_init_config, Args};

#[derive(Deserialize, Debug, Serialize)]
#[derive(Clone)]
pub struct Agent {
    pub id: String,
    pub docker: String,
    pub controller: String
}

impl Agent {

}

pub static CONFIG: OnceLock<Agent> = OnceLock::new();

pub fn get_config() -> &'static Agent {
    CONFIG.get_or_init(|| {
        check_config().expect("config initialized failed");
        let content = fs::read_to_string("config/agent.toml").expect("config.toml read failed");
        let config: Agent = toml::from_str(content.as_str()).expect("读取配置文件失败，请检查配置文件格式是否符合要求！");
        config
    })
}

pub fn set_config(agent: Agent) -> Result<(), Agent> {
    info!("config = {:?}", agent);
    CONFIG.set(agent)
}

fn check_config() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("config/agent.toml").exists() {
        let config = Agent {
            id: Uuid::new_v4().to_string(),
            docker: "http://localhost:2375".to_string(),
            controller: "http://localhost:5666".to_string()
        };
        let content = toml::to_string(&config)?;
        fs::write("config/agent.toml", content)?;
    }
    Ok(())
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    load_or_init_config(&args)?;
    Ok(())
}