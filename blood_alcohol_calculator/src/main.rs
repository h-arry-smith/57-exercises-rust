use std::io;
use std::io::Write;

enum Gender {
    Male,
    Female,
}

struct BAC {
    alcohol_in_oz: f64,
    body_weight_in_lb: f64,
    gender: Gender,
    hours_since_last_drink: f64,
}

impl BAC {
    fn new(
        alcohol_in_oz: f64,
        body_weight_in_lb: f64,
        gender: Gender,
        hours_since_last_drink: f64,
    ) -> Self {
        BAC {
            alcohol_in_oz,
            body_weight_in_lb,
            gender,
            hours_since_last_drink,
        }
    }

    fn bac(&self) -> f64 {
        (self.alcohol_in_oz * 5.14) / (self.body_weight_in_lb * self.alcohol_distribution_ratio())
            - (0.15 * self.hours_since_last_drink)
    }

    fn alcohol_distribution_ratio(&self) -> f64 {
        match self.gender {
            Gender::Male => 0.73,
            Gender::Female => 0.66,
        }
    }

    fn can_drive(&self) -> bool {
        self.bac() <= 0.08
    }
}

fn main() {
    let alcohol_in_oz = get_number("Total alcohol consumed (oz): ");
    let body_weight_in_lb = get_number("Body weight (lb): ");
    let gender = get_gender();
    let hours_since_last_drink = get_number("Hours since last drink: ");

    let bac = BAC::new(
        alcohol_in_oz,
        body_weight_in_lb,
        gender,
        hours_since_last_drink,
    );

    println!("Your BAC is {:.2}", bac.bac());

    if bac.can_drive() {
        println!("You can legally drive");
    } else {
        println!("You can not legally drive");
    }
}

fn get_gender() -> Gender {
    loop {
        print!("What is your gender? (M/F) ");
        io::stdout().flush().unwrap();

        let mut gender = String::new();

        io::stdin()
            .read_line(&mut gender)
            .expect("Failed to read line.");

        return match gender.trim().to_lowercase().as_str() {
            "m" => Gender::Male,
            "f" => Gender::Female,
            _ => continue,
        };
    }
}

fn get_number<T: std::str::FromStr>(question_text: &str) -> T {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: T = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
