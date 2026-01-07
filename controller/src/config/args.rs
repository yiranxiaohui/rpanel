use std::fs;
use std::fs::{create_dir_all};
use std::path::Path;
use clap::Parser;
use tracing::error;
use crate::config::{set_config, Controller};

#[derive(Parser, Debug)]
#[command(name = "controller")]
pub struct Args {

    /// Controller监听的端口
    #[arg(short, long, default_value = "15666")]
    port: Option<i16>,

    /// 指定配置文件，如未填写，则会生成默认的配置文件
    #[arg(long, default_value = "config/controller.toml")]
    config: String
}

pub fn init_controller_config(args: &Args) -> Result<Controller, Box<dyn std::error::Error>> {
    let path = Path::new(&args.config);

    // 1. 读取或初始化
    let mut config = if path.exists() {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content)?
    } else {
        Controller {
            port: 15666,
        }
    };

    // 2. 命令行参数覆盖配置文件
    if let Some(port) = &args.port {
        config.port = port.clone();
    }

    // 3. 如果文件不存在，或者被覆盖了，就写回
    if !path.exists() {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string(&config)?)?;
    }
    set_config(config.clone()).map_err(|e|{
        error!("设置Config失败！e => {:?}", e);
    }).unwrap();
    Ok(config)
}