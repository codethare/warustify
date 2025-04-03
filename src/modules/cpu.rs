use sysinfo::{System, SystemExt, ProcessorExt};

pub fn get_cpu_usage() -> f32 {
    let mut system = System::new_all();
    system.refresh_all();
    system.global_processor_info().cpu_usage()
}
