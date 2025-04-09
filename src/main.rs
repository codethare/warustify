mod monitor;
mod notifier;
mod events;

use tokio::sync::mpsc;
use events::Event;

#[tokio::main]
async fn main() {
    // 创建事件通道
    let (tx, mut rx) = mpsc::channel::<Event>(32);

    // 启动监控任务（该函数内部 spawn 了多个任务）
    monitor::start_monitor(tx).await;

    println!("事件驱动的系统监控程序启动...");

    // 主循环：接收事件并处理
    while let Some(event) = rx.recv().await {
        notifier::handle_event(event).await;
    }
}

