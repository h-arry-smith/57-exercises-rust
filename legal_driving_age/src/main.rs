use colored::Colorize;
use std::collections::HashMap;
use std::io;
use std::io::Write;

#[derive(PartialEq, PartialOrd)]
struct PositiveNumber(i32);

enum CanDriveResult {
    Yes,
    No,
}

impl std::fmt::Display for CanDriveResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CanDriveResult::Yes => write!(f, "{}", "YES".green().bold()),
            CanDriveResult::No => write!(f, "{}", "NO".red().bold()),
        }
    }
}

fn can_drive(age: &PositiveNumber, target: i32) -> CanDriveResult {
    if age >= &PositiveNumber(target) {
        CanDriveResult::Yes
    } else {
        CanDriveResult::No
    }
}

fn main() {
    let driving_ages = HashMap::from([("United Kingdom", 17), ("El Salvador", 15), ("France", 18)]);

    let age = get_positive_number("How old are you? ");

    for (&country, &target_age) in driving_ages.iter() {
        let result = can_drive(&age, target_age);
        println!("{:<15}: {}", country, result);
    }
}

fn get_positive_number(question_text: &str) -> PositiveNumber {
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

        if number > 0 {
            return PositiveNumber(number);
        }
    }
}
