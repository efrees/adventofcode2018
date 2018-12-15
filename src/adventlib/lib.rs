use std::fs::File;
use std::io::Read;

pub mod collections;
pub mod grid;

pub fn read_input_raw(filename: &str) -> String {
    let filename = "inputs/".to_owned() + filename;
    let mut file = File::open(filename).expect("Could not find input file");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Could not read file");
    return string;
}

pub fn read_input_lines(filename: &str) -> Vec<String> {
    let string = read_input_raw(filename);
    return string.lines().map(|x| x.to_string()).collect();
}

pub fn read_input_tokenized(filename: &str) -> Vec<String> {
    let string = read_input_raw(filename);
    return string.split_whitespace().map(|x| x.to_string()).collect();
}
