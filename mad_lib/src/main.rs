use std::io;
use std::io::Write;

fn main() {
    let noun = get_string("Enter a noun: ");
    let verb = get_string("Enter a verb: ");
    let adjective = get_string("Enter a adjective: ");
    let adverb = get_string("Enter a adverb: ");

    println!("Do you {verb} your {adjective} {noun} {adverb}? That's hilarious!");
}

fn get_string(question_text: &str) -> String {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    String::from(string.trim())
}
