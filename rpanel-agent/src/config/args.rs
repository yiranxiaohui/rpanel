use std::fs;
use std::fs::{create_dir_all};
use std::path::Path;
use clap::Parser;
use tracing::info;
use uuid::Uuid;
use crate::config::{set_config, Agent};

#[derive(Parser, Debug)]
#[command(name = "agent")]
pub struct Args {

    /// 节点唯一 ID（不传则自动生成 UUID）
    #[arg(long)]
    id: Option<String>,

    /// Docker HTTP服务地址
    #[arg(long, default_value = "http://localhost:2375")]
    url: Option<String>,

    /// 指定配置文件，如未填写，则会生成默认的配置文件
    #[arg(long, default_value = "config/agent.toml")]
    config: String
}

pub fn load_or_init_config(args: &Args) -> Result<Agent, Box<dyn std::error::Error>> {
    let path = Path::new(&args.config);

    // 1. 读取或初始化
    let mut config = if path.exists() {
        let content = fs::read_to_string(path)?;
        toml::from_str(&content)?
    } else {
        Agent {
            id: Uuid::new_v4().to_string(),
            url: "http://localhost:2375".to_string(),
        }
    };

    // 2. 命令行参数覆盖配置文件
    if let Some(id) = &args.id {
        config.id = id.clone();
    }

    if let Some(url) = &args.url {
        config.url = url.clone();
    }

    // 3. 如果文件不存在，或者被覆盖了，就写回
    if !path.exists() || args.id.is_some() || args.url.is_some() {
        if let Some(parent) = path.parent() {
            create_dir_all(parent)?;
        }
        fs::write(path, toml::to_string(&config)?)?;
    }
    set_config(config.clone()).unwrap();
    Ok(config)
}