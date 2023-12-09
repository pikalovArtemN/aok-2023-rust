use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn get_line_digits(line: String) -> Vec<char> {
    let mut number: Vec<char> = Vec::new();
    let numbers_as_strings =["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut i = 0;
    let line_len = line.len();
    while line_len > i {
        let character = line.chars().nth(i).unwrap();
        if character.is_ascii_digit() {
            number.push(character);
        }
        let sub_str = &line[i..];
        for str_number_map in numbers_as_strings.iter().enumerate() {
            if sub_str.starts_with(str_number_map.1) {
                let index = ((str_number_map.0 as i32) + 1).to_string();
                number.push(index.parse().unwrap());
                break;
            }
        }
        i += 1;
    }
    number
}

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let path = path.to_str().unwrap();
    let path = format!("{path}\\data.txt");

    let file = File::open(path)
        .expect("file not found!");
    let buf_reader = BufReader::new(file);
    let mut sum = 0;

    for line in buf_reader.lines() {
        let numbers = get_line_digits(line.unwrap());
        let number: i32 = format!("{}{}", numbers.first().unwrap(), numbers.last().unwrap()).parse().unwrap();
        sum += number;
    }

    println!("{}", sum);

    Ok(())
}
