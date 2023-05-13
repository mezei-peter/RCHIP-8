use crate::guestsystem::components::{
    cpu::{Cpu, CpuInst},
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

    pub fn decode(&self, raw: u16) -> CpuInst {
        match raw & 0xF000 {
            0x0000 => self.decode_0(raw),
            0x1000 => CpuInst::JmpNNN(self.make_nnn(raw)),
            0x2000 => CpuInst::SubRoutineNNN(self.make_nnn(raw)),
            0x3000 => CpuInst::SkipIfEqXNN(self.make_x(raw), self.make_nn(raw)),
            0x4000 => CpuInst::SkipIfNotEqXNN(self.make_x(raw), self.make_nn(raw)),
            0x5000 => {
                if raw & 0x000F == 0x0000 {
                    CpuInst::SkipIfEqXY(self.make_x(raw), self.make_y(raw))
                } else {
                    CpuInst::InvalidInstruction
                }
            }
            0x6000 => CpuInst::SetXNN(self.make_x(raw), self.make_nn(raw)),
            0x7000 => CpuInst::AddXNN(self.make_x(raw), self.make_nn(raw)),
            0x8000 => self.decode_8(raw),
            0x9000 => {
                if raw & 0x000F == 0x0000 {
                    CpuInst::SkipIfNotEqXY(self.make_x(raw), self.make_y(raw))
                } else {
                    CpuInst::InvalidInstruction
                }
            }
            0xA000 => CpuInst::SetIndexNNN(self.make_nnn(raw)),
            0xB000 => CpuInst::JmpOffsetNNN(self.make_nnn(raw)),
            0xC000 => CpuInst::RandomXNN(self.make_x(raw), self.make_nn(raw)),
            0xD000 => CpuInst::DisplayXYN(self.make_x(raw), self.make_y(raw), self.make_n(raw)),
            0xE000 => self.decode_e(raw),
            0xF000 => self.decode_f(raw),
            _ => CpuInst::InvalidInstruction,
        }
    }

    fn decode_0(&self, raw: u16) -> CpuInst {
        if raw & 0x00FF == 0x00EE {
            return CpuInst::SubReturn;
        }
        if raw & 0x00F0 == 0x00E0 {
            return CpuInst::Cls;
        }
        CpuInst::ExecMlrNNN(self.make_nnn(raw))
    }

    fn decode_8(&self, raw: u16) -> CpuInst {
        match raw & 0x000F {
            0x0000 => CpuInst::SetXY(self.make_x(raw), self.make_y(raw)),
            0x0001 => CpuInst::BitOrXY(self.make_x(raw), self.make_y(raw)),
            0x0002 => CpuInst::BitAndXY(self.make_x(raw), self.make_y(raw)),
            0x0003 => CpuInst::BitXorXY(self.make_x(raw), self.make_y(raw)),
            0x0004 => CpuInst::AddXY(self.make_x(raw), self.make_y(raw)),
            0x0005 => CpuInst::SubsFromLeftXY(self.make_x(raw), self.make_y(raw)),
            0x0006 => CpuInst::ShiftRightXY(self.make_x(raw), self.make_y(raw)),
            0x0007 => CpuInst::SubsFromRightXY(self.make_x(raw), self.make_y(raw)),
            0x000E => CpuInst::ShiftLeftXY(self.make_x(raw), self.make_y(raw)),
            _ => CpuInst::InvalidInstruction
        }
    }

    fn decode_e(&self, raw: u16) -> CpuInst {
        match raw & 0xF0FF {
            0xE09E => CpuInst::SkipIfKeyX(self.make_x(raw)),
            0xE0A1 => CpuInst::SkipIfNotKeyX(self.make_x(raw)),
            _ => CpuInst::InvalidInstruction
        }
    }

    fn decode_f(&self, raw: u16) -> CpuInst {
        match raw & 0xF0FF {
            0xF007 => CpuInst::SetRegToDelayX(self.make_x(raw)),
            0xF015 => CpuInst::SetDelayX(self.make_x(raw)),
            0xF018 => CpuInst::SetSoundX(self.make_x(raw)),
            0xF01E => CpuInst::AddToIndexX(self.make_x(raw)),
            0xF00A => CpuInst::WaitForKeyX(self.make_x(raw)),
            0xF029 => CpuInst::SetIndexToFontX(self.make_x(raw)),
            0xF033 => CpuInst::DecimalConversionX(self.make_x(raw)),
            0xF055 => CpuInst::StoreInMemoryX(self.make_x(raw)),
            0xF065 => CpuInst::LoadFromMemoryX(self.make_x(raw)),
            _ => CpuInst::InvalidInstruction
        }
    }

    pub fn make_x(&self, raw: u16) -> u8 {
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
