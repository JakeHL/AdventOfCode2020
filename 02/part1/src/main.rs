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

fn get_min_max(input: &String) -> (i32, i32) {
    let min_max_regex: Regex = Regex::new(r"[0-9]*-[0-9]*").unwrap();
    let range_string = get_regex_match(&min_max_regex, input);
    let range: Vec<&str> = range_string.split('-').collect();
    let min_string = range[0].parse::<i32>().unwrap();
    let max_string = range[1].parse::<i32>().unwrap();
    return (min_string, max_string);
}

fn get_char(input: &String) -> char {
    let char_regex: Regex = Regex::new(r"[a-zA-Z]:").unwrap();
    let range_string = get_regex_match(&char_regex, input).replace(':', "");
    let chars: Vec<char> = range_string.chars().collect();
    return chars[0];
}

fn get_password(input: &String) -> String {
    let char_regex: Regex = Regex::new(r"[a-zA-Z]*$").unwrap();
    let range_string = get_regex_match(&char_regex, input);
    return range_string;
}

fn is_password_valid(password: &String) -> bool {
    let (min, max) = get_min_max(password);
    let required = get_char(password);
    let password = get_password(password);
    let mut occurences: i32 = 0;
    for c in password.chars() {
        if c == required {
            occurences += 1;
        }
    }
    return occurences >= min && occurences <= max;
}

fn main() {
    let inputs = aoc_utils::read_file_to_array("./inputs.txt");
    let valid_passwords = inputs.iter().filter(|l| is_password_valid(l)).count();
    println!("{} valid passwords!", valid_passwords);
}
