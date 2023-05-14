use rand::Rng;
use sdl2::EventPump;

use crate::{config::CpuConfig, logic::interpreter::Interpreter};

use super::{
    display::DisplayScreen,
    keypad::Keypad,
    memory::{Memory, PROGRAM_ADDRESS},
};

const VARIABLE_REGISTER_COUNT: usize = 16;

#[derive(Debug)]
pub enum CpuInst {
    ExecMlrNNN(u16),
    Cls,
    JmpNNN(u16),
    SubRoutineNNN(u16),
    SubRoutineReturn,
    SkipIfEqXNN(u8, u8),
    SkipIfNotEqXNN(u8, u8),
    SkipIfEqXY(u8, u8),
    SkipIfNotEqXY(u8, u8),
    SetXNN(u8, u8),
    AddXNN(u8, u8),
    SetXY(u8, u8),
    BitOrXY(u8, u8),
    BitAndXY(u8, u8),
    BitXorXY(u8, u8),
    AddXY(u8, u8),
    SubtFromLeftXY(u8, u8),
    SubtFromRightXY(u8, u8),
    ShiftLeftXY(u8, u8),
    ShiftRightXY(u8, u8),
    SetIndexNNN(u16),
    JmpOffsetNNN(u16),
    RandomXNN(u8, u8),
    DisplayXYN(u8, u8, u8),
    SkipIfKeyX(u8),
    SkipIfNotKeyX(u8),
    SetRegToDelayX(u8),
    SetDelayX(u8),
    SetSoundX(u8),
    AddToIndexX(u8),
    WaitForKeyX(u8),
    SetIndexToFontX(u8),
    DecimalConversionX(u8),
    StoreInMemoryX(u8),
    LoadFromMemoryX(u8),
    InvalidInstruction,
}

pub struct Cpu {
    program_counter: u16,
    index_register: u16,
    variable_registers: [u8; VARIABLE_REGISTER_COUNT],
    delay_timer: u8,
    sound_timer: u8,
    config: CpuConfig,
}

impl Cpu {
    pub fn new(config: CpuConfig) -> Cpu {
        Cpu {
            program_counter: 0,
            index_register: 0,
            variable_registers: [0; VARIABLE_REGISTER_COUNT],
            delay_timer: 0,
            sound_timer: 0,
            config,
        }
    }

    pub fn point_pc_to_program(&mut self) {
        self.program_counter = PROGRAM_ADDRESS as u16;
    }

    pub fn set_flag_register(&mut self, arg: u8) {
        self.variable_registers[VARIABLE_REGISTER_COUNT - 1] = arg;
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

    pub fn decode(&self, raw_instruction: u16, interpreter: &Interpreter) -> CpuInst {
        interpreter.decode(raw_instruction)
    }

    pub fn execute(
        &mut self,
        instruction: &CpuInst,
        interpreter: &Interpreter,
        memory: &mut Memory,
        display: &mut DisplayScreen,
        keypad: &Keypad,
        event_pump: &mut EventPump,
    ) {
        match instruction {
            CpuInst::ExecMlrNNN(_) => {}
            CpuInst::Cls => display.clear_screen(),
            CpuInst::JmpNNN(nnn) => self.program_counter = interpreter.prev_pc(*nnn),
            CpuInst::SubRoutineNNN(nnn) => {
                memory.push_stack(self.program_counter);
                self.program_counter = interpreter.prev_pc(*nnn)
            }
            CpuInst::SubRoutineReturn => {
                self.program_counter = memory.pop_stack().expect("Error: Cannot from pop stack")
            }
            CpuInst::SkipIfEqXNN(x, nn) => {
                if self.variable_registers[*x as usize] == *nn {
                    self.program_counter = interpreter.next_pc(self.program_counter);
                }
            }
            CpuInst::SkipIfNotEqXNN(x, nn) => {
                if self.variable_registers[*x as usize] != *nn {
                    self.program_counter = interpreter.next_pc(self.program_counter);
                }
            }
            CpuInst::SkipIfEqXY(x, y) => {
                if self.variable_registers[*x as usize] == self.variable_registers[*y as usize] {
                    self.program_counter = interpreter.next_pc(self.program_counter);
                }
            }
            CpuInst::SkipIfNotEqXY(x, y) => {
                if self.variable_registers[*x as usize] != self.variable_registers[*y as usize] {
                    self.program_counter = interpreter.next_pc(self.program_counter);
                }
            }
            CpuInst::SetXNN(x, nn) => self.variable_registers[*x as usize] = *nn,
            CpuInst::AddXNN(x, nn) => {
                self.variable_registers[*x as usize] =
                    self.variable_registers[*x as usize].wrapping_add(*nn)
            }
            CpuInst::SetXY(x, y) => {
                self.variable_registers[*x as usize] = self.variable_registers[*y as usize]
            }
            CpuInst::BitOrXY(x, y) => {
                self.variable_registers[*x as usize] =
                    self.variable_registers[*x as usize] | self.variable_registers[*y as usize]
            }
            CpuInst::BitAndXY(x, y) => {
                self.variable_registers[*x as usize] =
                    self.variable_registers[*x as usize] & self.variable_registers[*y as usize]
            }
            CpuInst::BitXorXY(x, y) => {
                self.variable_registers[*x as usize] =
                    self.variable_registers[*x as usize] ^ self.variable_registers[*y as usize]
            }
            CpuInst::AddXY(x, y) => {
                if self.variable_registers[*x as usize]
                    .checked_add(self.variable_registers[*y as usize])
                    .is_none()
                {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize]
                    .wrapping_add(self.variable_registers[*y as usize])
            }
            CpuInst::SubtFromLeftXY(x, y) => {
                if self.variable_registers[*x as usize]
                    .checked_sub(self.variable_registers[*y as usize])
                    .is_none()
                {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize]
                    .wrapping_sub(self.variable_registers[*y as usize])
            }
            CpuInst::SubtFromRightXY(x, y) => {
                if self.variable_registers[*y as usize]
                    .checked_sub(self.variable_registers[*x as usize])
                    .is_none()
                {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
                self.variable_registers[*x as usize] = self.variable_registers[*y as usize]
                    .wrapping_sub(self.variable_registers[*x as usize])
            }
            CpuInst::ShiftLeftXY(x, y) => {
                if !self.config.modern_shift() {
                    self.variable_registers[*x as usize] = self.variable_registers[*y as usize];
                }
                let msb: bool = self.variable_registers[*x as usize] & 0x80 == 0x80;
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize] << 1;
                if msb {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
            }
            CpuInst::ShiftRightXY(x, y) => {
                if !self.config.modern_shift() {
                    self.variable_registers[*x as usize] = self.variable_registers[*y as usize];
                }
                let lsb: bool = self.variable_registers[*x as usize] & 1 == 1;
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize] >> 1;
                if lsb {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
            }
            CpuInst::SetIndexNNN(nnn) => self.index_register = *nnn,
            CpuInst::JmpOffsetNNN(nnn) => {
                if self.config.modern_jump_offset() {
                    let x: u8 = interpreter.make_x(*nnn);
                    let address = *nnn + self.variable_registers[x as usize] as u16;
                    self.program_counter = interpreter.prev_pc(address);
                } else {
                    let address = *nnn + self.variable_registers[0] as u16;
                    self.program_counter = interpreter.prev_pc(address);
                }
            }
            CpuInst::RandomXNN(x, nn) => {
                let random_number: u8 = rand::thread_rng().gen();
                self.variable_registers[*x as usize] = random_number & *nn;
            }
            CpuInst::DisplayXYN(x, y, n) => display.display(
                self.variable_registers[*x as usize],
                self.variable_registers[*y as usize],
                memory.get_heap_slice(self.index_register, *n as u16),
                self,
            ),
            CpuInst::SkipIfKeyX(_) => {}
            CpuInst::SkipIfNotKeyX(_) => {}
            CpuInst::SetRegToDelayX(_) => {}
            CpuInst::SetDelayX(_) => {}
            CpuInst::SetSoundX(_) => {}
            CpuInst::AddToIndexX(_) => {}
            CpuInst::WaitForKeyX(_) => {}
            CpuInst::SetIndexToFontX(_) => {}
            CpuInst::DecimalConversionX(_) => {}
            CpuInst::StoreInMemoryX(_) => {}
            CpuInst::LoadFromMemoryX(_) => {}
            CpuInst::InvalidInstruction => {}
        }
    }
}
