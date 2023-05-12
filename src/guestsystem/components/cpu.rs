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
}
