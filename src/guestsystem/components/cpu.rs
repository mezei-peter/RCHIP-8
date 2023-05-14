use std::time::{Duration, Instant};

use rand::Rng;
use sdl2::{event::Event, keyboard::Scancode, EventPump};

use crate::{config::CpuConfig, logic::interpreter::Interpreter};

use super::{
    display::DisplayScreen,
    keypad::Keypad,
    memory::{Memory, PROGRAM_ADDRESS},
};

const VARIABLE_REGISTER_COUNT: usize = 16;
const MAX_INDEX_REG_VAL: u16 = 0x0FFF;
const TIMER_HZ: f64 = 60.0;

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
    last_time: Instant,
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
            last_time: Instant::now(),
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
            self.program_counter = PROGRAM_ADDRESS as u16;
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
            CpuInst::JmpNNN(nnn) => self.program_counter = *nnn,
            CpuInst::SubRoutineNNN(nnn) => {
                memory.push_stack(self.program_counter);
                self.program_counter = *nnn
            }
            CpuInst::SubRoutineReturn => {
                self.program_counter = memory.pop_stack().expect("Error: Cannot pop from stack")
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
                if self.variable_registers[*x as usize] > self.variable_registers[*y as usize] {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize]
                    .wrapping_sub(self.variable_registers[*y as usize])
            }
            CpuInst::SubtFromRightXY(x, y) => {
                if self.variable_registers[*y as usize] > self.variable_registers[*x as usize] {
                    self.set_flag_register(1);
                } else {
                    self.set_flag_register(0);
                }
                self.variable_registers[*x as usize] = self.variable_registers[*x as usize]
                    .wrapping_sub(self.variable_registers[*y as usize])
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
                    self.program_counter = address;
                } else {
                    let address = *nnn + self.variable_registers[0] as u16;
                    self.program_counter = address;
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
            CpuInst::SkipIfKeyX(x) => {
                self.skip_if_key(*x as usize, interpreter, keypad, event_pump, true);
            }
            CpuInst::SkipIfNotKeyX(x) => {
                self.skip_if_key(*x as usize, interpreter, keypad, event_pump, false);
            }
            CpuInst::SetRegToDelayX(x) => self.variable_registers[*x as usize] = self.delay_timer,
            CpuInst::SetDelayX(x) => self.delay_timer = self.variable_registers[*x as usize],
            CpuInst::SetSoundX(x) => self.sound_timer = self.variable_registers[*x as usize],
            CpuInst::AddToIndexX(x) => self.add_to_index(*x as usize),
            CpuInst::WaitForKeyX(x) => {
                self.wait_for_key(*x as usize, keypad, event_pump);
            }
            CpuInst::SetIndexToFontX(x) => self.set_index_to_font(*x as usize, memory),
            CpuInst::DecimalConversionX(x) => self.store_three_decimal_digits(*x as usize, memory),
            CpuInst::StoreInMemoryX(x) => self.store_x_regs(*x as usize + 1, memory),
            CpuInst::LoadFromMemoryX(x) => self.load_x_regs(*x as usize + 1, memory),
            CpuInst::InvalidInstruction => {}
        }
    }

    fn skip_if_key(
        &mut self,
        reg_index: usize,
        interpreter: &Interpreter,
        keypad: &Keypad,
        event_pump: &mut EventPump,
        should_be_pressed: bool,
    ) {
        let key_val: u8 = self.variable_registers[reg_index];
        let key: Option<Scancode> = keypad.byte_to_scancode(key_val);
        if key.is_some() {
            for event in event_pump.poll_iter() {
                if let Event::KeyDown { scancode, .. } = event {
                    if should_be_pressed {
                        if scancode == key {
                            self.program_counter = interpreter.next_pc(self.program_counter);
                        }
                    } else {
                        if scancode != key {
                            self.program_counter = interpreter.next_pc(self.program_counter);
                        }
                    }
                }
            }
        } else if !should_be_pressed {
            self.program_counter = interpreter.next_pc(self.program_counter);
        }
    }

    fn wait_for_key(&mut self, x: usize, keypad: &Keypad, event_pump: &mut EventPump) {
        for event in event_pump.poll_iter() {
            if let Event::KeyDown { scancode, .. } = event {
                if scancode.is_some() {
                    let key_val = keypad.scancode_to_byte(&scancode.unwrap());
                    if key_val.is_none() {
                        self.program_counter = self.program_counter;
                        return;
                    }
                    self.variable_registers[x] = key_val.unwrap();
                } else {
                    self.program_counter = self.program_counter;
                }
            }
        }
    }

    fn add_to_index(&mut self, x: usize) {
        let vx: u16 = self.variable_registers[x] as u16;
        self.index_register += vx;
        if self.index_register > MAX_INDEX_REG_VAL {
            self.index_register %= self.index_register;
            if self.config.modern_index_addition() {
                self.set_flag_register(1);
            }
        }
    }

    pub fn operate_timers(&mut self) {
        let elapsed: Duration = Instant::elapsed(&self.last_time);
        if elapsed.as_secs_f64() >= 1.0 / TIMER_HZ {
            if self.delay_timer > 0 {
                self.delay_timer -= 1;
            }
            if self.sound_timer > 0 {
                self.sound_timer -= 1;
            }
            self.last_time = Instant::now();
        }
    }

    pub fn should_beep(&self) -> bool {
        self.sound_timer > 0
    }

    fn set_index_to_font(&mut self, x: usize, memory: &Memory) {
        let vx: u8 = self.variable_registers[x];
        let font_val: u8 = vx & 0x0F;
        self.index_register = memory.get_font(font_val);
    }

    fn store_three_decimal_digits(&self, x: usize, memory: &mut Memory) {
        let i: u16 = self.index_register;

        let mut value: u8 = self.variable_registers[x];
        let dig_3: u8 = value % 10;
        value -= dig_3;
        let dig_2: u8 = (value % 100) / 10;
        value -= dig_2 * 10;
        let dig_1: u8 = value / 100;

        memory.set_heap(i, dig_1);
        memory.set_heap(i + 1, dig_2);
        memory.set_heap(i + 2, dig_3);
    }

    fn store_x_regs(&mut self, reg_count: usize, memory: &mut Memory) {
        if self.config.modern_store_and_load() {
            for i in 0..reg_count {
                let vi: u8 = self.variable_registers[i];
                memory.set_heap(self.index_register + i as u16, vi);
            }
        } else {
            for i in 0..reg_count {
                let vi: u8 = self.variable_registers[i];
                memory.set_heap(self.index_register, vi);
                self.index_register += 1;
            }
        }
    }

    fn load_x_regs(&mut self, reg_count: usize, memory: &Memory) {
        if self.config.modern_store_and_load() {
            for i in 0..reg_count {
                let val: u8 = memory.at_address(self.index_register + i as u16);
                self.variable_registers[i] = val;
            }
        } else {
            for i in 0..reg_count {
                let val: u8 = memory.at_address(self.index_register);
                self.variable_registers[i] = val;
                self.index_register += 1;
            }
        }
    }
}
