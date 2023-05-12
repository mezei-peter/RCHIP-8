const FOUR_KIBI: usize = 4096;
const START_ADDRESS: u16 = 0x200;

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
}
