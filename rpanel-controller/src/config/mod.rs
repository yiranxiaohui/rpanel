use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct Config {
}

impl Config {

}


pub fn init() -> Config {
    let content = fs::read_to_string("config.toml").expect("读取配置文件失败，请检查配置文件是否存在！");
    let config: Config = toml::from_str(content.as_str()).expect("读取配置文件失败，请检查配置文件格式是否符合要求！");
    config
}