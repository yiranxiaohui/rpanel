use serde::{Deserialize, Serialize};
use sysinfo::{Disks, System};

#[derive(Debug, Serialize, Deserialize)]
pub struct Status {
    pub system: SystemStatus,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SystemStatus {
    pub cpu_usage: f32,
    pub mem_used: u64,
    pub mem_total: u64,
    pub disk_used: u64,
    pub disk_total: u64,
}

pub fn get_system_status() -> SystemStatus {
    let mut sys = System::new_all();

    // First we update all information of our `System` struct.
    sys.refresh_all();

    sys.refresh_memory();

    // CPU 使用率（取所有核的平均）
    let cpu_usage = sys
        .cpus()
        .iter()
        .map(|cpu| cpu.cpu_usage())
        .sum::<f32>() / sys.cpus().len() as f32;

    // 内存（sysinfo 单位是 KB）
    let mem_total = sys.total_memory() / 1024;
    let mem_used = (sys.total_memory() - sys.available_memory()) / 1024;

    // 磁盘（取所有磁盘总和，单位 GB）
    let mut disk_total = 0;
    let mut disk_used = 0;

    for disk in &Disks::new_with_refreshed_list() {
        disk_total += disk.total_space();
        disk_used += disk.total_space() - disk.available_space();
    }

    SystemStatus {
        cpu_usage,
        mem_used,
        mem_total,
        disk_used: disk_used / 1024 / 1024 / 1024,
        disk_total: disk_total / 1024 / 1024 / 1024,
    }
}