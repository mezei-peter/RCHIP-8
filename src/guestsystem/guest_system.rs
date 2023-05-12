use sdl2::{Sdl, event::Event, keyboard::Keycode};

use crate::logic::interpreter::Interpreter;

use super::components::{cpu::Cpu, display::DisplayScreen, keypad::Keypad, memory::Memory};

pub struct GuestSystem<'a> {
    memory: Memory,
    display: DisplayScreen<'a>,
    cpu: Cpu,
    keypad: Keypad<'a>,
    sdl_ctx: &'a Sdl,
}

impl<'a> GuestSystem<'a> {
    pub fn new(memory: Memory, cpu: Cpu, sdl_ctx: &'a Sdl) -> GuestSystem<'a> {
        GuestSystem {
            memory: memory,
            cpu: cpu,
            sdl_ctx: sdl_ctx,
            display: DisplayScreen::new(&sdl_ctx),
            keypad: Keypad::new(&sdl_ctx),
        }
    }

    pub fn run_program(&mut self, program: &Vec<u8>, interpreter: &Interpreter) {
        self.memory.load_fonts(interpreter.generate_fonts());
        self.memory.load_program(&program);

        let mut event_pump = self.sdl_ctx.event_pump().unwrap();
        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. } |
                    Event::KeyDown { keycode: Some(Keycode::Escape), .. } => break 'running,
                    _ => {}
                }
            }
        }
    }
}
