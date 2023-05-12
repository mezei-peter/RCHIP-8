use sdl2::Sdl;

pub struct Keypad<'a> {
    context: &'a Sdl,
}

impl<'a> Keypad<'a> {
    pub fn new(context: &Sdl) -> Keypad {
        Keypad { context }
    }
}
