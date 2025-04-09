/// 定义了监控过程中的各种事件
#[derive(Debug)]
pub enum Event {
    CpuHigh(f32),
    MemoryLow(u64),
    TemperatureHigh(f32),
    BatteryLow(u32), // 新增：电池电量低事件，百分比数值
}
