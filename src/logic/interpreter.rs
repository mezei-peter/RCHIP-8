use crate::guestsystem::components::{
    cpu::{Cpu, CpuInstruction},
    memory::Memory,
};

pub const FONT_SIZE: usize = 5;
pub const ALL_FONT_COUNT: usize = 16;

#[derive(Debug)]
pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Interpreter {
        Interpreter
    }

    pub fn generate_fonts(&self) -> [u8; FONT_SIZE * ALL_FONT_COUNT] {
        [
            0xF0, 0x90, 0x90, 0x90, 0xF0, //0
            0x20, 0x60, 0x20, 0x20, 0x70, //1
            0xF0, 0x10, 0xF0, 0x80, 0xF0, //2
            0xF0, 0x10, 0xF0, 0x10, 0xF0, //3
            0x90, 0x90, 0xF0, 0x10, 0x10, //4
            0xF0, 0x80, 0xF0, 0x10, 0xF0, //5
            0xF0, 0x80, 0xF0, 0x90, 0xF0, //6
            0xF0, 0x10, 0x20, 0x40, 0x40, //7
            0xF0, 0x90, 0xF0, 0x90, 0xF0, //8
            0xF0, 0x90, 0xF0, 0x10, 0xF0, //9
            0xF0, 0x90, 0xF0, 0x90, 0x90, //A
            0xE0, 0x90, 0xE0, 0x90, 0xE0, //B
            0xF0, 0x80, 0x80, 0x80, 0xF0, //C
            0xE0, 0x90, 0x90, 0x90, 0xE0, //D
            0xF0, 0x80, 0xF0, 0x80, 0xF0, //E
            0xF0, 0x80, 0xF0, 0x80, 0x80, //F
        ]
    }

    pub fn fetch(&self, memory: &Memory, address: u16) -> u16 {
        let byte_1: u8 = memory.at_address(address);
        let byte_2: u8 = memory.at_address(address + 1);
        ((byte_1 as u16) << 8) | byte_2 as u16
    }
    
    pub fn prev_pc(&self, current_address: u16) -> u16 {
        current_address - 2
    }

    pub fn next_pc(&self, current_address: u16) -> u16 {
        current_address + 2
    }

    pub fn decode(&self, raw: u16) -> CpuInstruction {
        match raw & 0xF000 {
            0x0000 => self.decode_0(raw),
            0x1000 => CpuInstruction::JmpNNN(self.make_nnn(raw)),
            0x2000 => CpuInstruction::SubRoutineNNN(self.make_nnn(raw)),
            0x3000 => CpuInstruction::SkipIfEqXNN(self.make_x(raw), self.make_nn(raw)),
            0x4000 => CpuInstruction::SkipIfNotEqXNN(self.make_x(raw), self.make_nn(raw)),
            0x5000 => {
                if raw & 0x000F == 0x0000 {
                    CpuInstruction::SkipIfEqXY(self.make_x(raw), self.make_y(raw))
                } else {
                    CpuInstruction::InvalidInstruction
                }
            }
            0x6000 => CpuInstruction::SetXNN(self.make_x(raw), self.make_nn(raw)),
            0x7000 => CpuInstruction::AddXNN(self.make_x(raw), self.make_nn(raw)),
            0x8000 => self.decode_8(raw),
            0x9000 => {
                if raw & 0x000F == 0x0000 {
                    CpuInstruction::SkipIfNotEqXY(self.make_x(raw), self.make_y(raw))
                } else {
                    CpuInstruction::InvalidInstruction
                }
            }
            0xA000 => CpuInstruction::SetIndexNNN(self.make_nnn(raw)),
            0xB000 => CpuInstruction::JmpOffsetNNN(self.make_nnn(raw)),
            0xC000 => CpuInstruction::RandomXNN(self.make_x(raw), self.make_nn(raw)),
            0xD000 => CpuInstruction::DisplayXYN(self.make_x(raw), self.make_y(raw), self.make_n(raw)),
            0xE000 => self.decode_e(raw),
            0xF000 => self.decode_f(raw),
            _ => CpuInstruction::InvalidInstruction,
        }
    }

    fn decode_0(&self, raw: u16) -> CpuInstruction {
        if raw & 0x00FF == 0x00EE {
            return CpuInstruction::SubReturn;
        }
        if raw & 0x00F0 == 0x00E0 {
            return CpuInstruction::Cls;
        }
        CpuInstruction::ExecMlrNNN(self.make_nnn(raw))
    }

    fn decode_8(&self, raw: u16) -> CpuInstruction {
        match raw & 0x000F {
            0x0000 => CpuInstruction::SetXY(self.make_x(raw), self.make_y(raw)),
            0x0001 => CpuInstruction::BitOrXY(self.make_x(raw), self.make_y(raw)),
            0x0002 => CpuInstruction::BitAndXY(self.make_x(raw), self.make_y(raw)),
            0x0003 => CpuInstruction::BitXorXY(self.make_x(raw), self.make_y(raw)),
            0x0004 => CpuInstruction::AddXY(self.make_x(raw), self.make_y(raw)),
            0x0005 => CpuInstruction::SubsFromLeftXY(self.make_x(raw), self.make_y(raw)),
            0x0006 => CpuInstruction::ShiftRightXY(self.make_x(raw), self.make_y(raw)),
            0x0007 => CpuInstruction::SubsFromRightXY(self.make_x(raw), self.make_y(raw)),
            0x000E => CpuInstruction::ShiftLeftXY(self.make_x(raw), self.make_y(raw)),
            _ => CpuInstruction::InvalidInstruction
        }
    }

    fn decode_e(&self, raw: u16) -> CpuInstruction {
        match raw & 0xF0FF {
            0xE09E => CpuInstruction::SkipIfKeyX(self.make_x(raw)),
            0xE0A1 => CpuInstruction::SkipIfNotKeyX(self.make_x(raw)),
            _ => CpuInstruction::InvalidInstruction
        }
    }

    fn decode_f(&self, raw: u16) -> CpuInstruction {
        match raw & 0xF0FF {
            0xF007 => CpuInstruction::SetRegToDelayX(self.make_x(raw)),
            0xF015 => CpuInstruction::SetDelayX(self.make_x(raw)),
            0xF018 => CpuInstruction::SetSoundX(self.make_x(raw)),
            0xF01E => CpuInstruction::AddToIndexX(self.make_x(raw)),
            0xF00A => CpuInstruction::WaitForKeyX(self.make_x(raw)),
            0xF029 => CpuInstruction::SetIndexToFontX(self.make_x(raw)),
            0xF033 => CpuInstruction::DecimalConversionX(self.make_x(raw)),
            0xF055 => CpuInstruction::StoreInMemoryX(self.make_x(raw)),
            0xF065 => CpuInstruction::LoadFromMemoryX(self.make_x(raw)),
            _ => CpuInstruction::InvalidInstruction
        }
    }

    fn make_x(&self, raw: u16) -> u8 {
        ((raw & 0x0F00) >> 8) as u8
    }

    fn make_y(&self, raw: u16) -> u8 {
        ((raw & 0x00F0) >> 4) as u8
    }

    fn make_n(&self, raw: u16) -> u8 {
        (raw & 0x000F) as u8
    }

    fn make_nn(&self, raw: u16) -> u8 {
        (raw & 0x00FF) as u8
    }

    fn make_nnn(&self, raw: u16) -> u16 {
        raw & 0x0FFF
    }

}
