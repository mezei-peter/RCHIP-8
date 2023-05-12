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

use logic::args_service::ArgsService;
use std::env;

extern crate sdl2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let args_service: ArgsService = ArgsService::new();
    args_service.run(&args);
}
