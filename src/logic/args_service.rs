use std::fs;

pub struct ArgsService;

impl ArgsService {
    pub fn new() -> ArgsService {
        ArgsService
    }

    pub fn find_path_arg(&self, args: &Vec<String>) -> String {
        let path: &str = match args.get(1) {
            Some(path) => path,
            None => "",
        };
        path.to_string()
    }

    pub fn read_rom(&self, rom_path: &str) -> Result<Vec<u8>, &str> {
        if rom_path == "" {
            return Err("No path provided for ROM.");
        }
        let file_contents_res = fs::read(rom_path);
        if file_contents_res.is_err() {
            return Err("File not found.");
        }
        Ok(file_contents_res.unwrap())
    }
}
