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

    pub fn next_pc(&self, current_address: u16) -> u16 {
        current_address + 2
    }

    pub fn decode(&self, raw_instruction: u16) -> CpuInstruction {
        match raw_instruction & 0xF000 {
            0x0000 => self.decode_0(raw_instruction),
            0x1000 => self.decode_1(raw_instruction),
            0x2000 => self.decode_2(raw_instruction),
            0x3000 => self.decode_3(raw_instruction),
            0x4000 => self.decode_4(raw_instruction),
            0x5000 => self.decode_5(raw_instruction),
            0x6000 => self.decode_6(raw_instruction),
            0x7000 => self.decode_7(raw_instruction),
            0x8000 => self.decode_8(raw_instruction),
            0x9000 => self.decode_9(raw_instruction),
            0xA000 => self.decode_A(raw_instruction),
            0xB000 => self.decode_B(raw_instruction),
            0xC000 => self.decode_C(raw_instruction),
            0xD000 => self.decode_D(raw_instruction),
            0xE000 => self.decode_E(raw_instruction),
            0xF000 => self.decode_F(raw_instruction),
            _ => CpuInstruction::InvalidInstruction
        }
    }

    fn decode_0(&self, raw_instruction: u16) -> CpuInstruction {
        if raw_instruction & 0x00FF == raw_instruction & 0x00EE {
            return CpuInstruction::SubReturn;
        }
        if raw_instruction & 0x00F0 == 0x00E0 {
            return CpuInstruction::Cls;
        }
        CpuInstruction::ExecMlrNNN(raw_instruction & 0x0FFF)
    }

    fn decode_1(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_2(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_3(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_4(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_5(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_6(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_7(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_8(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_9(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_A(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_B(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_C(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }
    
    fn decode_D(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_E(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }

    fn decode_F(&self, raw_instruction: u16) -> CpuInstruction {
        todo!()
    }
}
