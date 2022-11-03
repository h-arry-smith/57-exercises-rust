use std::io;
use std::io::Write;

fn main() {
    let mut input = String::new();

    loop {
        print!("What is the input string? ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut input)
            .expect("Please enter a string.");

        if input.trim() == "" {
            println!("Please enter a non-empty string.");
        } else {
            break;
        }
    }

    let input = input.trim();
    let length = input.len();

    println!("{input} has {length} characters.");
}
