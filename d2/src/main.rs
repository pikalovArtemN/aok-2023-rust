use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::env;

fn main() -> std::io::Result<()> {
    let path = env::current_dir()?;
    let path = path.to_str().unwrap();
    let path = format!("{path}\\data.txt");

    let file = File::open(path)
        .expect("file not found!");
    let buf_reader = BufReader::new(file);

    let mut result = 0;
    for line in buf_reader.lines() {
        let prompt_line = line.unwrap();
        let prompt_line = prompt_line.replace("Game ", "");
        let colon_char = prompt_line.find(":").unwrap();
        let prompt_line = &prompt_line[colon_char + 2..];
        let games = prompt_line.split("; ");

        let mut max_blue = 0;
        let mut max_red = 0;
        let mut max_green = 0;

        for game in games{
            let cubes = game.split(", ");
            for cube in cubes {
                if cube.contains("blue") {
                    let cube: i32 = cube.replace(" blue", "").parse().unwrap();
                    if cube > max_blue {
                        max_blue = cube;
                    }
                }
                if cube.contains("red") {
                    let cube: i32 = cube.replace(" red", "").parse().unwrap();
                    if cube > max_red {
                        max_red = cube;
                    }
                }
                if cube.contains("green") {
                    let cube: i32 = cube.replace(" green", "").parse().unwrap();
                    if cube > max_green {
                        max_green = cube;
                    }
                }
            }
        }
        result += max_blue * max_red * max_green;
        // let prompt_line = prompt_line.replace("green", "");
        // let prompt_line = prompt_line.replace("red", "");
        // let prompt_line = prompt_line.replace("blue", "");
    }
    println!("{}", result);

    Ok(())
}
