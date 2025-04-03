use std::time::Duration;
use std::thread;
use warustify::{get_battery_percentage, get_cpu_usage, get_available_memory, get_cpu_temperature, send_notification};

fn main() {
    loop {
        // 检查 CPU 占用率
        let cpu_usage = get_cpu_usage();
        if cpu_usage > 90.0 {
            send_notification("CPU 使用率过高", &format!("CPU 占用率达到 {}%", cpu_usage));
        }

        // 检查内存
        let available_memory = get_available_memory();
        if available_memory < 7 * 1024 * 1024 * 1024 { // 2GB
            send_notification("内存不足", &format!("可用内存为 {} MB", available_memory / 1024 / 1024));
        }

        // 检查电池电量
        if let Some(percentage) = get_battery_percentage() {
            if percentage < 82.0 {
                send_notification("电池电量低", &format!("电池电量为 {}%", percentage));
            }
        }

        // 检查 CPU 温度
        if let Some(temperature) = get_cpu_temperature() {
            if temperature > 70.0 {
                send_notification("CPU 温度过高", &format!("CPU 温度为 {}°C", temperature));
            }
        }

        // 每10秒检查一次
        thread::sleep(Duration::from_secs(10));
    }
}
