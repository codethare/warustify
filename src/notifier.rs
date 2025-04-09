use crate::events::Event;
use notify_rust::{Notification, Timeout};

/// 根据接收到的事件发送相应的桌面通知
pub async fn handle_event(event: Event) {
    match event {
        Event::CpuHigh(usage) => {
            println!("CPU 使用率过高: {:.2}%", usage);
            send_notification("CPU 警告", &format!("CPU 使用率超过 {}%: {:.2}%", 90.0, usage));
        },
        Event::MemoryLow(mem_bytes) => {
            let mem_gb = mem_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
            println!("可用内存不足: {:.2} GB", mem_gb);
            send_notification("内存警告", &format!("可用内存低于 2.0GB: {:.2} GB", mem_gb));
        },
        Event::TemperatureHigh(temp) => {
            println!("CPU 温度过高: {:.1}°C", temp);
            send_notification("温度警告", &format!("CPU 温度超过 {}°C: {:.1}°C", 70.0, temp));
        },
        Event::BatteryLow(battery) => {
            println!("电池电量低: {}%", battery);
            send_notification("电池警告", &format!("电池电量低于 {}%: {}%", 82, battery));
        },
    }
}

/// 使用 notify‑rust 发送桌面通知
fn send_notification(summary: &str, body: &str) {
    if let Err(e) = Notification::new()
        .summary(summary)
        .body(body)
        .icon("dialog-warning")
        .timeout(Timeout::Milliseconds(15000))
        .show() 
    {
        eprintln!("通知发送失败: {:?}", e);
    }
}

