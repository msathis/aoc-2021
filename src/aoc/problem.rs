use std::fs::File;
use std::io::Read;

pub trait Problem {
    fn part_one(&self) -> String;
    fn part_two(&self) -> String;

    fn input_as_string(&self, path: String) -> String {
        let mut file = match File::open(path) {
            Err(e) => panic!("File is not found, {}", e),
            Ok(f) => f,
        };

        let mut buf = String::new();
        match file.read_to_string(&mut buf) {
            Ok(_) => println!("Input read successfully"),
            Err(e) => panic!("File is not readable, {}", e),
        }
        buf
    }

    fn input_as_vec(&self, path: String) -> Vec<String> {
        let string = self.input_as_string(path);
        string.lines().map(|d| d.to_string()).collect()
    }
}
