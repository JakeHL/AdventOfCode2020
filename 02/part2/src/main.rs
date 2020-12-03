use aoc_utils;
use regex::Regex;

fn get_regex_match(reg: &Regex, input: &String) -> String {
    let matching = reg.find(&input).unwrap();
    let range_start = matching.start();
    let range_end = matching.end();
    let mut result = String::new();
    result.push_str(&input[range_start..range_end]);
    return result;
}

fn get_positions(input: &String) -> (usize, usize) {
    let min_max_regex: Regex = Regex::new(r"[0-9]*-[0-9]*").unwrap();
    let range_string = get_regex_match(&min_max_regex, input);
    let positions: Vec<&str> = range_string.split('-').collect();
    let first_position = positions[0].parse::<usize>().unwrap();
    let second_position = positions[1].parse::<usize>().unwrap();
    return (first_position, second_position);
}

fn get_char(input: &String) -> char {
    let char_regex: Regex = Regex::new(r"[a-zA-Z]:").unwrap();
    let range_string = get_regex_match(&char_regex, input).replace(':', "");
    let chars: Vec<char> = range_string.chars().collect();
    return chars[0];
}

fn get_password(input: &String) -> String {
    let char_regex: Regex = Regex::new(r" [a-zA-Z]*$").unwrap();
    let range_string = get_regex_match(&char_regex, input).replace(' ', "");
    return range_string;
}

fn is_password_valid(password: &String) -> bool {
    let (first, second) = get_positions(password);
    let required = get_char(password);
    let password = get_password(password);
    let password_chars: Vec<char> = password.chars().collect();
    println!("{:#?}", password_chars);
    let pos_1_match = password_chars[first - 1] == required;
    let pos_2_match = password_chars[second - 1] == required;
    return pos_1_match ^ pos_2_match;
}

fn main() {
    let inputs = aoc_utils::read_file_to_array("./inputs.txt");
    let valid_passwords = inputs.iter().filter(|l| is_password_valid(l)).count();
    println!("{} valid passwords!", valid_passwords);
}
