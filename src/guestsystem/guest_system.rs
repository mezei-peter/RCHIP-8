use super::components::{cpu::Cpu, display::DisplayScreen, memory::Memory};

pub struct GuestSystem {
    memory: Memory,
    display: DisplayScreen,
    cpu: Cpu,
}
