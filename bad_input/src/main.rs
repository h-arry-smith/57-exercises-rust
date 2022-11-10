use std::io;
use std::io::Write;

fn get_positive_number(question_text: &str) -> f64 {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut answer = String::new();

        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read a line.");

        let number: f64 = match answer.trim().parse() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("You must enter a valid number.");
                continue;
            }
        };

        if number > 0.0 {
            return number;
        } else {
            eprintln!("You must enter a number greater than 0.");
            continue;
        }
    }
}

fn main() {
    let rate = get_positive_number("What is the rate of return? ");
    let years = 72.0 / rate;

    println!(
        "It will take {:.1} years to double your initial investment.",
        years
    );
}
