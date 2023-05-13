pub mod config;
mod logic {
    pub mod args_service;
    pub mod interpreter;
}
mod guestsystem {
    pub mod components {
        pub mod cpu;
        pub mod display;
        pub mod keypad;
        pub mod memory;
    }
    pub mod guest_system;
}

use config::EmulatorConfig;
use guestsystem::{
    components::{cpu::Cpu, display::DisplayScreen, keypad::Keypad, memory::Memory},
    guest_system::GuestSystem,
};
use logic::{args_service::ArgsService, interpreter::Interpreter};
use sdl2::Sdl;
use std::env;

extern crate sdl2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let args_service: ArgsService = ArgsService::new();
    let path: String = args_service.find_path_arg(&args);
    let mut emulator_config: EmulatorConfig = EmulatorConfig::default();
    if args_service.find_config_arg(&args) {
        emulator_config = args_service.prompt_config();
    }
    dbg!(emulator_config);

    let sdl_context: Sdl = sdl2::init().unwrap();
    let mut guest_system: GuestSystem = GuestSystem::new(Memory::new(), Cpu::new(), &sdl_context);
    let interpreter: Interpreter = Interpreter::new();

    match args_service.read_rom(&path) {
        Ok(rom_bytes) => guest_system.run_program(&rom_bytes, &interpreter),
        Err(msg) => println!("{}", msg),
    }
}
