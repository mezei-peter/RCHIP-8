use sdl2::{
    event::Event,
    keyboard::{Keycode, Scancode},
    Sdl,
};

use crate::logic::interpreter::Interpreter;

use super::components::{
    cpu::{Cpu, CpuInstruction},
    display::DisplayScreen,
    keypad::Keypad,
    memory::Memory,
};

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
        self.cpu.point_pc_to_program();

        let mut event_pump = self.sdl_ctx.event_pump().unwrap();
        'running: loop {
            let raw_instruction: u16 = self.cpu.fetch(&self.memory, &interpreter);
            let instruction: CpuInstruction = self.cpu.decode(raw_instruction, interpreter);
            self.cpu.execute(
                &instruction,
                &mut self.memory,
                &mut self.display,
                &self.keypad,
                &mut event_pump,
            );
            for event in event_pump.poll_iter() {
                match event {
                    Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    }
                    | Event::Quit { .. } => break 'running,
                    _ => self.handle_keys(&event),
                }
            }
        }
    }

    fn handle_keys(&mut self, event: &Event) {
        match event {
            Event::KeyDown {
                scancode: Some(scode),
                ..
            } => {
                let byte_val = self.keypad.scancode_to_byte(scode);
                if byte_val.is_none() {
                    return;
                } else {
                    println!("VALID KEY!");
                }
            }
            _ => {}
        }
    }
}
