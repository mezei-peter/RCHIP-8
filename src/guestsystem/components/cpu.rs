use crate::logic::interpreter::Interpreter;

use super::memory::{Memory, PROGRAM_ADDRESS};

const VARIABLE_REGISTER_COUNT: usize = 16;

pub struct Cpu {
    program_counter: u16,
    index_register: u16,
    variable_registers: [u8; VARIABLE_REGISTER_COUNT],
    delay_timer: u8,
    sound_timer: u8,
}

impl Cpu {
    pub fn new() -> Cpu {
        Cpu {
            program_counter: 0,
            index_register: 0,
            variable_registers: [0; VARIABLE_REGISTER_COUNT],
            delay_timer: 0,
            sound_timer: 0,
        }
    }

    pub fn point_pc_to_program(&mut self) {
        self.program_counter = PROGRAM_ADDRESS as u16;
    }

    pub fn get_pc(&self) -> u16 {
        self.program_counter
    }

    pub fn set_pc(&mut self, address: u16, max_address: u16) {
        if address > max_address {
            return;
        }
        self.program_counter = address
    }

    pub fn fetch(&mut self, memory: &Memory, interpreter: &Interpreter) -> u16 {
        let instruction: u16 = interpreter.fetch(memory, self.program_counter);
        self.set_pc(
            interpreter.next_pc(self.program_counter),
            memory.get_heap_size() as u16 - 1,
        );
        instruction
    }
}
