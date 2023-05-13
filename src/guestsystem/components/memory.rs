use crate::logic::interpreter::{ALL_FONT_COUNT, FONT_SIZE};

const FOUR_KIBI: usize = 4096;
pub const PROGRAM_ADDRESS: usize = 0x200;
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

    pub fn load_program(&mut self, program: &[u8]) {
        self.load(&program, PROGRAM_ADDRESS);
    }

    pub fn load_fonts(&mut self, fonts: [u8; FONT_SIZE * ALL_FONT_COUNT]) {
        self.load(&fonts, FONTS_ADDRESS);
    }

    fn load(&mut self, buffer: &[u8], address: usize) {
        let length: usize = buffer.len();
        let mut i: usize = 0;
        while i < length {
            let font_byte: u8 = buffer[i];
            self.heap[address + i] = font_byte;
            i += 1;
        }
    }

    pub fn at_address(&self, address: u16) -> u8 {
        self.heap[address as usize]
    }

    pub fn get_heap_size(&self) -> usize {
        self.heap.len()
    }
}
