use clap::Parser;
use rand::prelude::*;
use std::collections::HashMap;
use words::get_word_list;
mod words;

#[derive(Parser, Debug)]
#[command(name = "dicew")]
#[command(author = "Corban Procuniar <corbanpro@gmail.com>")]
#[command(version = "1.0.1")]
#[command(about = "generate diceware password", long_about = None)]
struct Args {
    #[arg(value_name = "NUM WORDS", default_value_t = 5)]
    num_words: u8,
}

fn main() {
    let args = Args::parse();
    // make a diceware generator
    let input = get_word_list();
    let lines: Vec<(&str, &str)> = input
        .trim()
        .split("\n")
        .map(|line| {
            let mut parts = line.trim().split("|");
            (parts.next().unwrap().trim(), parts.next().unwrap().trim())
        })
        .collect();
    let words: HashMap<&str, &str> = HashMap::from_iter(lines);

    let password = generate_password(&words, args.num_words);

    for word in &password {
        println!("{} ", word);
    }

    println!("\n{}", password.join(""));
}

fn generate_password(words: &HashMap<&str, &str>, num_words: u8) -> Vec<String> {
    let mut password = vec![];

    for _ in 0..num_words {
        let mut roll = String::new();
        for _ in 0..5 {
            roll += &roll_die().to_string()
        }
        let word = words.get(roll.as_str()).unwrap();
        password.push(word.to_string());
    }

    password
}

fn roll_die() -> u8 {
    let mut rng = rand::rng();
    rng.random_range(1..=6)
}
