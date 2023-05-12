mod logic {
    pub mod args_service;
}
mod guestsystem {
    pub mod components {
        pub mod cpu;
        pub mod display;
        pub mod memory;
    }
    pub mod guest_system;
}

use guestsystem::{guest_system::GuestSystem, components::{memory::Memory, cpu::Cpu, display::DisplayScreen}};
use logic::args_service::ArgsService;
use std::env;

extern crate sdl2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let guest_system: GuestSystem = GuestSystem::new(Memory::new(), DisplayScreen::new(), Cpu::new());
    let args_service: ArgsService = ArgsService::new();
    let path: String = args_service.find_path_arg(&args);
    match args_service.read_rom(&path) {
        Ok(rom_bytes) => guest_system.load_program_to_memory(&rom_bytes),
        Err(msg) => println!("{}", msg)
    }
}
