use sysinfo::{System, SystemExt};

pub fn get_available_memory() -> u64 {
    let mut system = System::new_all();
    system.refresh_all();
    system.available_memory()
}
