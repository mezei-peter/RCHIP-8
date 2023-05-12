mod logic {
    pub mod args_service;
}

use logic::{args_service::ArgsService};
use std::env;

extern crate sdl2;

pub fn main() {
    let args: Vec<String> = env::args().collect();
    let args_service: ArgsService = ArgsService::new();
    args_service.run(&args);
}
