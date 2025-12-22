mod args;

use std::fs;
use std::path::Path;
use std::sync::OnceLock;
use clap::Parser;
use serde::{Deserialize, Serialize};
use tracing::info;
use crate::config::args::{init_controller_config, Args};

#[derive(Deserialize, Debug, Serialize, Clone)]
pub struct Controller {
    pub(crate) port: i16
}

impl Controller {

}

pub static CONFIG: OnceLock<Controller> = OnceLock::new();

pub fn set_config(config: Controller) -> Result<(), Controller>{
    info!("config = {:?}", config);
    CONFIG.set(config)
}

pub fn get_config() -> &'static Controller {
    CONFIG.get_or_init(|| {
        check_config().expect("config initialized failed");
        let content = fs::read_to_string("config/controller.toml").expect("config.toml read failed");
        let config: Controller = toml::from_str(content.as_str()).expect("读取配置文件失败，请检查配置文件格式是否符合要求！");
        config
    })
}

fn check_config() -> Result<(), Box<dyn std::error::Error>> {
    if !Path::new("config/controller.toml").exists() {
        let config = Controller {port: 15666};
        let content = toml::to_string(&config)?;
        fs::write("config/controller.toml", content)?;
    }
    Ok(())
}

pub fn init_config() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    init_controller_config(&args)?;
    Ok(())
}