use clap::Parser;
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

#[tokio::main]
async fn main() {
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

    let password = generate_password(&words, args.num_words).await;

    for word in &password {
        println!("{} ", word);
    }

    println!("\n{}", password.join(""));
}

async fn generate_password(words: &HashMap<&str, &str>, num_words: u8) -> Vec<String> {
    let mut password = vec![];

    for _ in 0..num_words {
        let roll = roll_die().await;
        let word = words.get(roll.as_str()).unwrap();
        password.push(word.to_string());
    }

    password
}

async fn roll_die() -> String {
    let url = "https://www.randomnumberapi.com/api/v1.0/random?min=1&max=6&count=5";
    let response = reqwest::get(url).await.unwrap();
    let rolls: Vec<i32> = response.json().await.unwrap();
    rolls
        .iter()
        .map(|roll| roll.to_string())
        .collect::<Vec<String>>()
        .join("")
}
