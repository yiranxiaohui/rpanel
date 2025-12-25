use serde::{Deserialize, Serialize};
use sysinfo::System;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AgentRegisterRequest {
    pub name: String,
    pub host_name: Option<String>,
    pub ip_address: Option<String>,
    pub os_info: Option<String>,
    pub version: Option<String>,
    pub tags: Option<Vec<String>>,
}

impl AgentRegisterRequest {
    pub fn new(name: String) -> Self {
        let host_name = System::host_name();
        let os_info = System::long_os_version();
        let version = option_env!("CARGO_PKG_VERSION").map(|s| s.to_string());

        AgentRegisterRequest {
            name,
            host_name,
            ip_address: None, // IP地址获取相对复杂，这里暂时留空或由 Controller 获取连接IP
            os_info,
            version,
            tags: None,
        }
    }
}
