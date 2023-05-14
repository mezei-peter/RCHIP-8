use sdl2::{keyboard::Scancode, Sdl};

pub struct Keypad<'a> {
    context: &'a Sdl,
    current_key: Option<Scancode>,
}

impl<'a> Keypad<'a> {
    pub fn new(context: &Sdl) -> Keypad {
        Keypad { context, current_key: None }
    }

    //  QWERTY  EMULATED
    // 1 2 3 4 | 1 2 3 C
    // Q W E R | 4 5 6 D
    // A S D F | 7 8 9 E
    // Z X C V | A 0 B F
    pub fn scancode_to_byte(&self, scancode: &Scancode) -> Option<u8> {
        match scancode {
            Scancode::Num1 => Some(0x1),
            Scancode::Num2 => Some(0x2),
            Scancode::Num3 => Some(0x3),
            Scancode::Num4 => Some(0xC),
            Scancode::Q => Some(0x4),
            Scancode::W => Some(0x5),
            Scancode::E => Some(0x6),
            Scancode::R => Some(0xD),
            Scancode::A => Some(0x7),
            Scancode::S => Some(0x8),
            Scancode::D => Some(0x9),
            Scancode::F => Some(0xE),
            Scancode::Z => Some(0xA),
            Scancode::X => Some(0x0),
            Scancode::C => Some(0xB),
            Scancode::V => Some(0xF),
            _ => None,
        }
    }

    pub fn byte_to_scancode(&self, key_val: u8) -> Option<Scancode> {
        match key_val {
            0x0 => Some(Scancode::X),
            0x1 => Some(Scancode::Num1),
            0x2 => Some(Scancode::Num2),
            0x3 => Some(Scancode::Num3),
            0x4 => Some(Scancode::Q),
            0x5 => Some(Scancode::W),
            0x6 => Some(Scancode::E),
            0x7 => Some(Scancode::A),
            0x8 => Some(Scancode::S),
            0x9 => Some(Scancode::D),
            0xA => Some(Scancode::Z),
            0xB => Some(Scancode::C),
            0xC => Some(Scancode::Num4),
            0xD => Some(Scancode::R),
            0xE => Some(Scancode::F),
            0xF => Some(Scancode::V),
            _ => None,
        }
    }

    pub fn set_current_key(&mut self, scancode: Option<Scancode>) {
        self.current_key = scancode
    }

    pub fn same_current_key_val(&self, key_val: u8) -> bool {
        match self.current_key {
            Some(scancode) => {
                let current_val: Option<u8> = self.scancode_to_byte(&scancode);
                if current_val.is_none() {
                    return false;
                }
                current_val.unwrap() == key_val
            }
            None => false
        }
    }

    pub fn current_key(&self) -> Option<Scancode> {
        self.current_key
    }
}
