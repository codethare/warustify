use sysinfo::{System, SystemExt, ComponentExt};

pub fn get_cpu_temperature() -> Option<f32> {
    let mut system = System::new_all();
    system.refresh_all();
    for component in system.components() {
        if component.label().to_lowercase().contains("cpu") {
            return Some(component.temperature());
        }
    }
    None
}
