use std::io;
use std::io::Write;

fn main() {
    let principal = get_float("Enter the principal: ");
    let rate = get_float("Enter the rate (%): ") / 100.0;
    let years = get_int("Enter the number of years: ");

    for year in 1..=years {
        let final_amount = simple_interest(principal, rate, year);

        println!(
            "After {year} years at {}%, the investment will be worth ${:.2}",
            rate * 100.0,
            final_amount
        );
    }
}

fn simple_interest(principal: f64, rate: f64, years: i32) -> f64 {
    principal * (1.0 + (rate * years as f64))
}

fn get_int(question_text: &str) -> i32 {
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

fn get_float(question_text: &str) -> f64 {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
