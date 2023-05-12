use super::components::{cpu::Cpu, display::DisplayScreen, memory::Memory};

pub struct GuestSystem {
    memory: Memory,
    display: DisplayScreen,
    cpu: Cpu,
}

impl GuestSystem {
    pub fn new(memory: Memory, display: DisplayScreen, cpu: Cpu) -> GuestSystem {
        GuestSystem {
            memory,
            display,
            cpu,
        }
    }

    pub fn load_program_to_memory(&self, bytes: &Vec<u8>) {
        todo!()
    }
}
