use chrono::Datelike;
use std::io;
use std::io::Write;

fn main() {
    let age = get_number("What is your current age? ");
    let retirement_age = get_number("At what age would you like to retire? ");

    let years_until_retirement = retirement_age - age;
    let current_year = chrono::Utc::now().year();

    if years_until_retirement < 0 {
        println!("You can already retire.");
    } else {
        println!("You have {years_until_retirement} years left until you can retire.");

        let retirement_year = current_year + years_until_retirement;

        println!("It's {current_year}, so you can retire in {retirement_year}");
    }
}

fn get_number(question_text: &str) -> i32 {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
