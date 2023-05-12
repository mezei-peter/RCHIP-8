mod logic {
    pub mod args_service;
    pub mod interpreter;
}
mod guestsystem {
    pub mod components {
        pub mod cpu;
        pub mod display;
        pub mod memory;
    }
    pub mod guest_system;
}

use guestsystem::{
    components::{cpu::Cpu, display::DisplayScreen, memory::Memory},
    guest_system::GuestSystem,
};
use logic::{args_service::ArgsService, interpreter::Interpreter};
use std::env;

extern crate sdl2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let args_service: ArgsService = ArgsService::new();
    let guest_system: GuestSystem =
        GuestSystem::new(Memory::new(), DisplayScreen::new(), Cpu::new());
    let interpreter: Interpreter = Interpreter::new();
    
    let path: String = args_service.find_path_arg(&args);
    match args_service.read_rom(&path) {
        Ok(rom_bytes) => guest_system.run_program(&rom_bytes, &interpreter),
        Err(msg) => println!("{}", msg),
    }
}
