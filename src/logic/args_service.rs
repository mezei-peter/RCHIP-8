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
    }
}
