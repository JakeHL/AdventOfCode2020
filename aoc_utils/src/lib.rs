use std::fs::File;
use std::io::{self, BufRead};

pub fn read_file_to_array(path: &str) -> Vec<String> {
    let lines = parse_file(path);
    let lines = match lines {
        Ok(l) => l,
        Err(error) => panic!("error {}", error),
    };
    return read_lines(lines);
}

fn read_lines(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<String> {
    let mut inputs: Vec<String> = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            inputs.push(l)
        }
    }
    return inputs;
}

fn parse_file(path: &str) -> io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let file = File::open(path)?;
    return Ok(io::BufReader::new(file).lines());
}
