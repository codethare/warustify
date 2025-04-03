use std::fs::File;
use std::io::Read;

pub fn get_battery_percentage() -> Option<f32> {
    let battery_path = "/sys/class/power_supply/BAT0/";
    let status_path = format!("{}status", battery_path);
    let energy_now_path = format!("{}energy_now", battery_path);
    let energy_full_path = format!("{}energy_full", battery_path);

    let mut status_file = File::open(status_path).ok()?;
    let mut status = String::new();
    status_file.read_to_string(&mut status).ok()?;
    let status = status.trim();

    if status != "Discharging" {
        return None;
    }

    let mut energy_now_file = File::open(energy_now_path).ok()?;
    let mut energy_now_str = String::new();
    energy_now_file.read_to_string(&mut energy_now_str).ok()?;
    let energy_now: f32 = energy_now_str.trim().parse().ok()?;

    let mut energy_full_file = File::open(energy_full_path).ok()?;
    let mut energy_full_str = String::new();
    energy_full_file.read_to_string(&mut energy_full_str).ok()?;
    let energy_full: f32 = energy_full_str.trim().parse().ok()?;

    Some((energy_now / energy_full) * 100.0)
}
