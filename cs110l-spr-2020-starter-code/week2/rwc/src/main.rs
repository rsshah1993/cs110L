use std::env;
use std::process;
use std::io::{self, BufRead};
use std::fs::File; 


fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Too few arguments.");
        process::exit(1);
    }
    let filename = &args[1];
    // Your code here :)
    let file = File::open(filename).expect("invalid file!");
    let mut lines = Vec::new();
    for line in io::BufReader::new(file).lines() {
        let line_str = line.expect("invalid line!");
        lines.push(line_str);
    };
    let num_lines = lines.len();

    let mut word_count = 0;
    let mut char_count = 0;

    for line in lines {
        let splits = line.split_whitespace();
        for split in splits {
            word_count += 1;
            for _ in split.chars() {
                char_count += 1;
            }
        }
    }

    println!("Your file has {} lines", num_lines);
    println!("Your file has {} words", word_count);
    println!("Your file has {} characters", char_count);
}