use std::fs;
use std::sync::OnceLock;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
#[derive(Clone)]
pub struct Config {
    pub url: String,
}

impl Config {

}

pub static CONFIG: OnceLock<Config> = OnceLock::new();

pub fn get_config() -> &'static Config {
    CONFIG.get_or_init(|| {
        let content = fs::read_to_string("config.toml").expect("读取配置文件失败，请检查配置文件是否存在！");
        let config: Config = toml::from_str(content.as_str()).expect("读取配置文件失败，请检查配置文件格式是否符合要求！");
        config
    })
}