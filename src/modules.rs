use sysinfo::{System, SystemExt, ProcessorExt, ComponentExt};
use tokio::sync::mpsc::Sender;
use std::time::Duration;
use tokio::time;
use crate::events::Event;

// 阈值设置
const CPU_THRESHOLD: f32 = 90.0;
const MEMORY_THRESHOLD_BYTES: u64 = 2 * 1024 * 1024 * 1024; // 2GB
const TEMP_THRESHOLD: f32 = 70.0;

/// 异步启动监控任务，定时采集系统数据并判断是否触发事件
pub async fn start_monitor(tx: Sender<Event>) {
    // 创建一个异步定时器，每 60 秒触发一次
    let mut interval = time::interval(Duration::from_secs(60));
    // 初始化系统信息
    let mut sys = System::new_all();
    loop {
        // 等待下一个定时器 tick
        interval.tick().await;

        // 更新系统所有数据
        sys.refresh_all();

        // 检查 CPU 使用率（取所有处理器平均值）
        let cpu_usage = sys
            .processors()
            .iter()
            .map(|p| p.cpu_usage())
            .sum::<f32>() / sys.processors().len() as f32;
        if cpu_usage > CPU_THRESHOLD {
            let _ = tx.send(Event::CpuHigh(cpu_usage)).await;
        }

        // 检查内存：available_memory 返回的是字节数
        let available_memory = sys.available_memory();
        if available_memory < MEMORY_THRESHOLD_BYTES {
            let _ = tx.send(Event::MemoryLow(available_memory)).await;
        }

        // 检查温度：刷新组件信息后，遍历寻找 CPU 组件
        sys.refresh_components();
        for comp in sys.components() {
            if comp.label().to_lowercase().contains("cpu") {
                let temp = comp.temperature();
                if temp > TEMP_THRESHOLD {
                    let _ = tx.send(Event::TemperatureHigh(temp)).await;
                }
                break;
            }
        }
    }
}

