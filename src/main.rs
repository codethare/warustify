use notify_rust::Notification;
use std::{fs, thread, time::Duration};
use sysinfo::{CpuExt, System, SystemExt};

fn main() {
    let mut sys = System::new_all();

    loop {
        sys.refresh_all();

        // 检测 CPU 使用率，若使用率超过 80% 则发出通知
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        if cpu_usage > 80.0 {
            notify("⚠ CPU使用率高", &format!("CPU使用率已达 {:.1}%", cpu_usage));
        }

        // 检测可用内存，若少于 2048 MB 则发出通知
        let mem_mb = sys.available_memory() / 1024;
        if mem_mb < 2048 {
            notify("⚠ 内存不足", &format!("可用内存仅剩 {} MB", mem_mb));
        }

        // 检测电池电量，低于 82% 时发出通知
        if let Some(battery) = read_battery_percentage() {
            if battery < 82.0 {
                notify("⚠ 电池电量低", &format!("当前电量为 {:.1}%", battery));
            }
        }

        // 每 10 秒检测一次
        thread::sleep(Duration::from_secs(10));
    }
}

/// 发送通知，同时输出日志到 stdout 以便 journald 收录
fn notify(summary: &str, body: &str) {
    println!("[通知] {} - {}", summary, body);
    let _ = Notification::new().summary(summary).body(body).show();
}

/// 尝试从系统文件中读取电池电量百分比  
/// 此处假设电池设备为 /sys/class/power_supply/BAT0/capacity  
fn read_battery_percentage() -> Option<f32> {
    let capacity_file = "/sys/class/power_supply/BAT0/capacity";
    match fs::read_to_string(capacity_file) {
        Ok(content) => content.trim().parse::<f32>().ok(),
        Err(e) => {
            eprintln!("读取电池信息失败: {}", e);
            None
        }
    }
}

