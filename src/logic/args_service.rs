use std::fs;

pub struct ArgsService;

impl ArgsService {
    pub fn new() -> ArgsService {
        ArgsService
    }

    pub fn run(&self, args: &Vec<String>) {
        let rom_path: &str = match args.get(1) {
            Some(path) => path,
            None => "",
        };
        if rom_path == "" {
            println!("No path provided for ROM.");
            return;
        }
        println!("ROM path: {}", rom_path);
        let file_contents_res = fs::read_to_string(rom_path);
        if file_contents_res.is_err() {
            println!("File not found.");
            return;
        }
        let file_contents = file_contents_res.unwrap();
        let rom: &[u8] = file_contents.as_bytes();
    }
}
