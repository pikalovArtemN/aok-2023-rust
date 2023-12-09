use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn main()  -> std::io::Result<()> {
    let path = env::current_dir()?;
    let path = path.to_str().unwrap();
    let path = format!("{path}\\data.txt");
    println!("{}", path);

    let file = File::open(path)
        .expect("file not found!");
    let  buf_reader = BufReader::new(file);
    let mut sum = 0;

    for line in buf_reader.lines() {
        let mut number = String::new();
        for character in line.unwrap().chars() {
            if character.is_ascii_digit() {
                number.push(character);
            }
        }
        let len = number.len();

        let number: i32 = format!("{}{}", &number[..1],&number[len-1..]).parse().unwrap();
        sum += number;
    }

    println!("{}", sum);

    Ok(())
}
