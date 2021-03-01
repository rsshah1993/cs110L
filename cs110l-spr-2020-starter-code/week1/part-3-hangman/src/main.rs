// Simple Hangman Program
// User gets five incorrect guesses
// Word chosen randomly from words.txt
// Inspiration from: https://doc.rust-lang.org/book/ch02-00-guessing-game-tutorial.html
// This assignment will introduce you to some fundamental syntax in Rust:
// - variable declaration
// - string manipulation
// - conditional statements
// - loops
// - vectors
// - files
// - user input
// We've tried to limit/hide Rust's quirks since we'll discuss those details
// more in depth in the coming lectures.
extern crate rand;
use rand::Rng;
use std::fs;
use std::io;
use std::io::Write;

const NUM_INCORRECT_GUESSES: u32 = 5;
const WORDS_PATH: &str = "words.txt";

fn pick_a_random_word() -> String {
    let file_string = fs::read_to_string(WORDS_PATH).expect("Unable to read file.");
    let words: Vec<&str> = file_string.split('\n').collect();
    String::from(words[rand::thread_rng().gen_range(0, words.len())].trim())
}

fn main() {
    let mut errors = 0;
    let secret_word = pick_a_random_word();
    // Note: given what you know about Rust so far, it's easier to pull characters out of a
    // vector than it is to pull them out of a string. You can get the ith character of
    // secret_word by doing secret_word_chars[i].
    let secret_word_chars: Vec<char> = secret_word.chars().collect();
    let tot_chars = secret_word_chars.len();
    let mut user_word_chars: Vec<String> = vec!["-".to_string(); tot_chars];
    // Uncomment for debugging:
        println!("random word: {}", secret_word);

    // Your code here! :)
    println!("Welcome to CS110L Hangman!");

    while errors <= NUM_INCORRECT_GUESSES {
        print!("Please guess a letter: ");
        io::stdout()
            .flush()
            .expect("Error flushing stdout.");
        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("Error reading line.");

        assert_eq!(guess.chars().count(), 2);
        let guess_char = guess.chars().next().unwrap();

        if secret_word_chars.iter().any(|&x| x == guess_char) {
            let idx = secret_word_chars.iter().position(|&x| x == guess_char).unwrap();
            println!("{}", secret_word_chars[idx].to_string());
            user_word_chars[idx] = secret_word_chars[idx].to_string();
            println!("{:?}", user_word_chars);

        }
        else {
            errors += 1;
            println!("Incorrect guess {} out of {} guesses remaining!", NUM_INCORRECT_GUESSES - errors, NUM_INCORRECT_GUESSES);
        }
    }

}
