use sdl2::EventPump;

use crate::logic::interpreter::{self, Interpreter};

use super::{
    display::DisplayScreen,
    keypad::Keypad,
    memory::{Memory, PROGRAM_ADDRESS},
};

const VARIABLE_REGISTER_COUNT: usize = 16;

#[derive(Debug)]
pub enum CpuInstruction {
    ExecMlrNNN(u16),
    Cls,
    JmpNNN(u16),
    SubRoutineNNN(u16),
    SubReturn,
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
    SubsFromLeftXY(u8, u8),
    SubsFromRightXY(u8, u8),
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

    pub fn decode(&self, raw_instruction: u16, interpreter: &Interpreter) -> CpuInstruction {
        interpreter.decode(raw_instruction)
    }

    pub fn execute(
        &mut self,
        instruction: &CpuInstruction,
        interpreter: &Interpreter,
        memory: &mut Memory,
        display: &mut DisplayScreen,
        keypad: &Keypad,
        event_pump: &mut EventPump,
    ) {
        match instruction {
            CpuInstruction::ExecMlrNNN(_) => {},
            CpuInstruction::Cls => display.clear_screen(),
            CpuInstruction::JmpNNN(nnn) => self.program_counter = interpreter.prev_pc(*nnn),
            CpuInstruction::SubRoutineNNN(_) => {},
            CpuInstruction::SubReturn => {},
            CpuInstruction::SetIndexNNN(nnn) => self.index_register = *nnn,
            CpuInstruction::SkipIfEqXNN(_, _) => {},
            CpuInstruction::SkipIfNotEqXNN(_, _) => {},
            CpuInstruction::SetXNN(x, nn) => self.variable_registers[*x as usize] = *nn,
            CpuInstruction::AddXNN(x, nn) => self.variable_registers[*x as usize] += *nn,
            CpuInstruction::SkipIfEqXY(_, _) => {},
            CpuInstruction::SkipIfNotEqXY(_, _) => {},
            CpuInstruction::SetXY(_, _) => {},
            CpuInstruction::BitOrXY(_, _) => {},
            CpuInstruction::BitAndXY(_, _) => {},
            CpuInstruction::BitXorXY(_, _) => {},
            CpuInstruction::AddXY(_, _) => {},
            CpuInstruction::SubsFromLeftXY(_, _) => {},
            CpuInstruction::SubsFromRightXY(_, _) => {},
            CpuInstruction::ShiftLeftXY(_, _) =>{},
            CpuInstruction::ShiftRightXY(_, _) => {},
            CpuInstruction::JmpOffsetNNN(_) => {},
            CpuInstruction::RandomXNN(_, _) => {},
            CpuInstruction::DisplayXYN(x, y, n) => {
                display.display(
                    self.variable_registers[*x as usize],
                    self.variable_registers[*y as usize],
                    memory.get_heap_slice(self.index_register, *n as u16),
                    self,
                )
            }
            CpuInstruction::SkipIfKeyX(_) => {},
            CpuInstruction::SkipIfNotKeyX(_) => {},
            CpuInstruction::SetRegToDelayX(_) => {},
            CpuInstruction::SetDelayX(_) => {},
            CpuInstruction::SetSoundX(_) => {},
            CpuInstruction::AddToIndexX(_) => {},
            CpuInstruction::WaitForKeyX(_) => {},
            CpuInstruction::SetIndexToFontX(_) => {},
            CpuInstruction::DecimalConversionX(_) => {},
            CpuInstruction::StoreInMemoryX(_) => {},
            CpuInstruction::LoadFromMemoryX(_) => {},
            CpuInstruction::InvalidInstruction => {},
        }
    }
}
