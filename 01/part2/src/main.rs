use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let lines = read_lines("./src/input.txt");
    let lines = match lines {
        Ok(l) => l,
        Err(error) => panic!("error {}", error),
    };
    let inputs = read_lines_to_ints(lines);
    for x in &inputs {
        for y in &inputs {
            for z in &inputs {
                let sum = x + y + z;
                if sum == 2020 {
                    println!(
                        "FOUND: {0}, {1}, {2}. {0} * {1} * {2} == {3}",
                        x,
                        y,
                        z,
                        x * y * z
                    );
                    return;
                }
            }
        }
    }
}

fn read_lines_to_ints(lines: std::io::Lines<std::io::BufReader<std::fs::File>>) -> Vec<i32> {
    let mut inputs: Vec<i32> = Vec::new();
    for line in lines {
        if let Ok(l) = line {
            inputs.push(l.parse::<i32>().unwrap())
        }
    }
    return inputs;
}

fn read_lines(path: &str) -> io::Result<std::io::Lines<std::io::BufReader<std::fs::File>>> {
    let file = File::open(path)?;
    return Ok(io::BufReader::new(file).lines());
}
