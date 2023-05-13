#[derive(Debug)]
pub struct EmulatorConfig {
    modern_shift: bool,
    modern_jump_offset: bool,
    modern_store_and_load: bool,
}

impl EmulatorConfig {
    pub fn new(
        modern_shift: bool,
        modern_jump_offset: bool,
        modern_store_and_load: bool,
    ) -> EmulatorConfig {
        EmulatorConfig {
            modern_shift,
            modern_jump_offset,
            modern_store_and_load,
        }
    }

    pub fn default() -> EmulatorConfig {
        EmulatorConfig {
            modern_shift: true,
            modern_jump_offset: false,
            modern_store_and_load: true,
        }
    }
}
