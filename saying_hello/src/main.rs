use std::io;
use std::io::Write;

fn main() {
    print!("What is your name? ");
    io::stdout().flush().unwrap();

    let mut name = String::new();

    io::stdin()
        .read_line(&mut name)
        .expect("Please enter a string!");

    let name = name.trim();

    let output = match name {
        "Harry" => format!("Ah, {name}, nice to see you again!"),
        "Mary" => format!("Not you again, {name}!"),
        _ => format!("Hello {name}, nice to meet you!"),
    };

    println!("{}", output);
}
