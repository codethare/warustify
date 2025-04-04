use std::time::Duration;
use std::thread;
use warustify::{get_battery_percentage, get_cpu_usage, get_available_memory, get_cpu_temperature, send_notification};

fn main() {
    loop {
        // Check CPU usage
        let cpu_usage = get_cpu_usage();
        if cpu_usage > 90.0 {
            send_notification("CPU !!!", &format!("CPU {}%", cpu_usage));
        }

        // Check memory
        let available_memory = get_available_memory();
        let threshold: u64 = 2 * 1024 * 1024 * 1024; // 2GB in bytes
        if available_memory > threshold {
            let available_memory_mb = available_memory / 1024 / 1024;
            send_notification("MEM !!!", &format!("MEM {} MB", available_memory_mb));
        }

        // Check battery percentage
        if let Some(percentage) = get_battery_percentage() {
            if percentage < 82.0 {
                send_notification("BATTERY !!!", &format!("BATTERY {}%", percentage));
            }
        }

        // Check CPU temperature
        if let Some(temperature) = get_cpu_temperature() {
            if temperature > 70.0 {
                send_notification("CPU TEMP !!!", &format!("CPU TEMP {}°C", temperature));
            }
        }

        // Sleep for 10 seconds before checking again
        thread::sleep(Duration::from_secs(10));
    }
}
