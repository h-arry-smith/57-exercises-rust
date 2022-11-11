use std::io::{self, Write};

use rand::Rng;

fn main() {
    let answers = ["Yes", "No", "Maybe", "Ask again later."];

    get_input_and_do_nothing("What is your question? ");

    let mut rng = rand::thread_rng();

    let answer = answers[rng.gen_range(0..answers.len())];

    println!("{answer}");
}

fn get_input_and_do_nothing(question_text: &str) {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut String::new())
        .expect("Can't read a line.");
}
