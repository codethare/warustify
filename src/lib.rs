pub mod modules;

pub use modules::battery::get_battery_percentage;
pub use modules::cpu::get_cpu_usage;
pub use modules::memory::get_available_memory;
pub use modules::temperature::get_cpu_temperature;
pub use modules::notification::send_notification;
