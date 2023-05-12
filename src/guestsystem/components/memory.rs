use crate::logic::interpreter::{ALL_FONT_COUNT, FONT_SIZE};

const FOUR_KIBI: usize = 4096;
const PROGRAM_ADDRESS: usize = 0x200;
const FONTS_ADDRESS: usize = 0x050;

#[derive(Debug)]
pub struct Memory {
    heap: [u8; FOUR_KIBI],
    stack: Vec<u16>,
}

impl Memory {
    pub fn new() -> Memory {
        Memory {
            heap: [0; FOUR_KIBI],
            stack: Vec::new(),
        }
    }

    pub fn load_program(&self, program: &[u8]) {
        todo!()
    }

    pub fn load_fonts(&mut self, fonts: [u8; FONT_SIZE * ALL_FONT_COUNT]) {
        let length: usize = fonts.len();
        let mut i: usize = 0;
        while i < length {
            let font_byte: u8 = fonts[i];
            self.heap[FONTS_ADDRESS + i] = font_byte;
            i += 1;
        }
        dbg!(self);
    }
}
