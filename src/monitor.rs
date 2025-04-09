use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt};
use tokio::sync::mpsc::Sender;
use tokio::time::{self, Duration};
use crate::events::Event;
use std::fs;

// 阈值设置
const CPU_THRESHOLD: f32 = 90.0;
// 注意：sysinfo 的 available_memory() 单位为 KB，转换为字节后做比较
const MEMORY_THRESHOLD_BYTES: u64 = 2 * 1024 * 1024 * 1024; // 2GB
const TEMP_THRESHOLD: f32 = 70.0;
const BATTERY_THRESHOLD: u32 = 82;

/// 启动所有监控任务。每项监控在各自的间隔时间内采集数据，并发送对应事件到通道。
pub async fn start_monitor(tx: Sender<Event>) {
    // 克隆通道发送器，用于各个子任务
    let cpu_tx = tx.clone();
    let mem_tx = tx.clone();
    let temp_tx = tx.clone();
    let bat_tx = tx.clone();

    // CPU 检查：每 30 秒检查一次
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        // 创建系统实例，只刷新 CPU 信息即可
        let mut sys = System::new();
        loop {
            interval.tick().await;
            sys.refresh_cpu();
            let processors = sys.processors();
            if processors.is_empty() {
                continue;
            }
            let avg_usage = processors.iter()
                .map(|p| p.cpu_usage())
                .sum::<f32>() / (processors.len() as f32);
            if avg_usage > CPU_THRESHOLD {
                let _ = cpu_tx.send(Event::CpuHigh(avg_usage)).await;
            }
        }
    });

    // 内存检查：每 30 秒检查一次
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(30));
        let mut sys = System::new();
        loop {
            interval.tick().await;
            sys.refresh_memory();
            let available_kb = sys.available_memory();
            let available_bytes = available_kb * 1024; // 转换为字节
            if available_bytes < MEMORY_THRESHOLD_BYTES {
                let _ = mem_tx.send(Event::MemoryLow(available_bytes)).await;
            }
        }
    });

    // CPU 温度检查：每 60 秒检查一次
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(60));
        let mut sys = System::new();
        loop {
            interval.tick().await;
            sys.refresh_components();
            for comp in sys.components() {
                if comp.label().to_lowercase().contains("cpu") {
                    let temp = comp.temperature();
                    if temp > TEMP_THRESHOLD {
                        let _ = temp_tx.send(Event::TemperatureHigh(temp)).await;
                    }
                    break; // 找到一个 CPU 组件后退出循环
                }
            }
        }
    });

    // 电池检查：每 60 分钟检查一次
    tokio::spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3600)); // 3600秒 = 60分钟
        loop {
            interval.tick().await;
            if let Ok(capacity_str) = fs::read_to_string("/sys/class/power_supply/BAT0/capacity") {
                if let Ok(capacity) = capacity_str.trim().parse::<u32>() {
                    if capacity < BATTERY_THRESHOLD {
                        let _ = bat_tx.send(Event::BatteryLow(capacity)).await;
                    }
                }
            }
        }
    });
}

