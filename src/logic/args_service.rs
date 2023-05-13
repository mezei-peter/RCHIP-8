use std::{fs, io::stdin};

use crate::config::EmulatorConfig;

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

    pub fn find_config_arg(&self, args: &[String]) -> bool {
        if args.len() >= 3 {
            if args[2] == "--config" {
                println!("CONFIG");
                return true;
            }
        }
        return false;
    }

    pub fn prompt_config(&self) -> EmulatorConfig {
        println!("-- Configuration Options --");
        println!();
        println!("  Would you like to allow: ");
        let modern_shift: bool =
            self.prompt_config_option("  -> modern bitwise shift behaviour? (Y/N)");
        let modern_jump_offset: bool =
            self.prompt_config_option("  -> modern jump offset instruction? (Y/N)");
        let modern_store_and_load: bool = self.prompt_config_option(
            "  -> modern method of storing data in/loading data from memory? (Y/N)",
        );
        EmulatorConfig::new(modern_shift, modern_jump_offset, modern_store_and_load)
    }

    fn read_line(&self) -> String {
        let mut s = String::new();
        stdin().read_line(&mut s).expect("Invalid string.");
        self.trim_crlf(&mut s);
        s
    }

    fn trim_crlf(&self, s: &mut String) {
        if s.ends_with('\n') {
            s.pop();
            if s.ends_with('\r') {
                s.pop();
            }
        }
    }

    fn prompt_config_option(&self, question: &str) -> bool {
        loop {
            println!("{}", question);
            let s = self.read_line().to_lowercase();
            if s == "y" {
                return true;
            } else if s == "n" {
                return false;
            } else {
                println!("  Please answer either 'Y' or 'N'");
            }
        }
    }
}
