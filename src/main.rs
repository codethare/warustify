// main.rs
use log::{error, info, warn};
use notify_rust::Notification;
use std::time::Duration; // <-- 移除 thread
use sysinfo::{CpuExt, System, SystemExt}; // <-- 引入 SystemExt trait
use zbus::{dbus_proxy, Connection, Result as ZbusResult}; // <-- 移除 Proxy

// --- D-Bus 代理定義 ---
#[dbus_proxy(
    interface = "org.freedesktop.UPower",
    default_service = "org.freedesktop.UPower",
    default_path = "/org/freedesktop/UPower"
)]
trait UPower {
    /// EnumerateDevices method
    fn enumerate_devices(&self) -> ZbusResult<Vec<zbus::zvariant::OwnedObjectPath>>;
}

// 為 PowerDevice 代理明確指定不使用預設值
#[dbus_proxy(
    interface = "org.freedesktop.UPower.Device",
    assume_defaults = false // <-- 告知宏我們不依賴預設 path/service
)]
trait PowerDevice {
    /// Percentage property
    #[dbus_proxy(property)]
    fn percentage(&self) -> ZbusResult<f64>;

    /// Type property
    #[dbus_proxy(property)]
    fn type_(&self) -> ZbusResult<u32>;

    /// IsRechargeable property
    #[dbus_proxy(property)]
    fn is_rechargeable(&self) -> ZbusResult<bool>;
}
// --- D-Bus 代理定義結束 ---

#[tokio::main]
async fn main() {
    env_logger::init();
    notify("測試", "通知系統工作正常");

    let connection = match Connection::session().await {
        Ok(conn) => conn,
        Err(e) => {
            error!("無法連接到 D-Bus Session Bus: {}", e);
            return;
        }
    };
    info!("成功連接到 D-Bus Session Bus");

    // 現在 new_all() 和 refresh_all() 應該可以工作了
    let mut sys = System::new_all();
    info!("開始監控系統資源...");

    loop {
        // refresh_all() 可能也來自 SystemExt，所以引入 trait 很重要
        sys.refresh_all();

        // --- CPU 檢測 ---
        // global_cpu_info() 也來自 SystemExt
        let cpu_usage = sys.global_cpu_info().cpu_usage();
        if cpu_usage > 80.0 {
            let msg = format!("CPU 使用率已達 {:.1}%", cpu_usage);
            warn!("{}", msg);
            notify("⚠️ CPU 使用率高", &msg);
        }

        // --- 記憶體檢測 ---
        let available_mem_mb = sys.available_memory() / 1024 / 1024;
        if available_mem_mb < 2048 {
            let msg = format!("可用記憶體僅剩 {} MB", available_mem_mb);
            warn!("{}", msg);
            notify("⚠️ 記憶體不足", &msg);
        }

        // --- 電池檢測 (使用 D-Bus) ---
        match get_battery_percentage_dbus(&connection).await {
            Ok(Some(percentage)) => {
                if percentage < 82.0 {
                    let msg = format!("當前電量為 {:.1}%", percentage);
                    warn!("{}", msg);
                    notify("⚠️ 電池電量低", &msg);
                }
            }
            Ok(None) => {
                // info!("未偵測到電池設備。");
            }
            Err(e) => {
                error!("查詢電池狀態時出錯: {}", e);
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        }

        // 使用 Tokio 的異步 sleep
        tokio::time::sleep(Duration::from_secs(10)).await;
    }
}

fn notify(summary: &str, body: &str) {
    if let Err(e) = Notification::new().summary(summary).body(body).show() {
        error!("發送桌面通知失敗: {}", e);
    }
}

async fn get_battery_percentage_dbus(connection: &Connection) -> ZbusResult<Option<f64>> {
    let upower_proxy = UPowerProxy::new(connection).await?;
    // info!("成功創建 UPower D-Bus 代理"); // 成功後無需每次都記錄

    let devices = match upower_proxy.enumerate_devices().await {
        Ok(d) => d,
        Err(e) => {
            error!("調用 UPower EnumerateDevices 失敗: {}", e);
            return Err(e); // 直接返回錯誤
        }
    };
    // info!("獲取到 {} 個電源設備", devices.len());

    for device_path in devices {
        // 為每個設備創建代理，這裡需要處理路徑無效的可能性
        let device_proxy = match PowerDeviceProxy::builder(connection)
            .path(&device_path)? // 傳入設備路徑引用
            .build()
            .await
        {
            Ok(p) => p,
            Err(e) => {
                warn!("為路徑 {:?} 創建設備代理失敗: {}", device_path, e);
                continue; // 跳過這個設備
            }
        };

        // 獲取屬性時也需要處理錯誤
        let device_type = match device_proxy.type_().await {
            Ok(t) => t,
            Err(e) => {
                warn!("無法獲取設備 {:?} 的類型: {}", device_proxy.path(), e);
                continue;
            }
        };
        let is_rechargeable = match device_proxy.is_rechargeable().await {
            Ok(r) => r,
            Err(e) => {
                warn!("無法獲取設備 {:?} 的可充電狀態: {}", device_proxy.path(), e);
                continue;
            }
        };

        if device_type == 2 && is_rechargeable {
            // 2 代表電池
            // info!("找到電池設備: {:?}", device_proxy.path());
            return match device_proxy.percentage().await {
                Ok(p) => {
                    // info!("獲取到電池百分比: {:.1}%", p);
                    Ok(Some(p))
                }
                Err(e) => {
                    error!("無法獲取電池 {:?} 的百分比: {}", device_proxy.path(), e);
                    Err(e) // 將錯誤向上傳播
                }
            };
        }
    }

    // info!("未找到可充電電池設備"); // 找不到時不一定是警告，可能就是沒有
    Ok(None)
}

