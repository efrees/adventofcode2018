use std::fs::File;
use std::io::Read;

pub fn read_input_lines(filename: &str) -> Vec<String> {
    let filename = "inputs/".to_owned() + filename;
    let mut file = File::open(filename).expect("Could not find input file");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Could not read file");

    return string.lines().map(|x| x.to_string()).collect();
}

pub fn read_input_tokenized(filename: &str) -> Vec<String> {
    let filename = "inputs/".to_owned() + filename;
    let mut file = File::open(filename).expect("Could not find input file");
    let mut string = String::new();
    file.read_to_string(&mut string)
        .expect("Could not read file");

    return string.split_whitespace().map(|x| x.to_string()).collect();
}
