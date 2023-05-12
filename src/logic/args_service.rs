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
        let file_contents_res = fs::read(rom_path);
        if file_contents_res.is_err() {
            println!("File not found.");
            return;
        }
        let rom = file_contents_res.unwrap();
    }
}
