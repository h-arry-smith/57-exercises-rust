use std::io;
use std::io::Write;

fn main() {
    print!("What is the quote? ");
    io::stdout().flush().unwrap();

    let quote = get_string();

    print!("What is the author? ");
    io::stdout().flush().unwrap();

    let author = get_string();

    println!("{author} says, \"{quote}\"");
}

fn get_string() -> String {
    let mut string = String::new();

    io::stdin()
        .read_line(&mut string)
        .expect("Please enter a string");

    String::from(string.trim())
}
