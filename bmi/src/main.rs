use std::io::{self, Write};

struct Person {
    height: f64,
    weight: f64,
}

impl Person {
    fn new(height: f64, weight: f64) -> Self {
        Person { height, weight }
    }
}

struct BMI {
    pub bmi: f64,
    pub category: BMICategory,
}

impl BMI {
    fn new(person: &Person) -> Self {
        let bmi = Self::calculate_bmi(&person.height, &person.weight);

        BMI {
            bmi,
            category: Self::category(bmi),
        }
    }

    fn calculate_bmi(height: &f64, weight: &f64) -> f64 {
        (weight / (height * height)) * 703.0
    }

    fn category(bmi: f64) -> BMICategory {
        if bmi < 18.5 {
            BMICategory::Underweight
        } else if bmi > 25.0 {
            BMICategory::Overweight
        } else {
            BMICategory::Normal
        }
    }
}

enum BMICategory {
    Underweight,
    Normal,
    Overweight,
}

fn main() {
    let height = get_number("Height (in): ");
    let weight = get_number("Weight (lb): ");

    let person = Person::new(height, weight);
    let bmi = BMI::new(&person);

    println!("Your BMI is {:.1}", bmi.bmi);

    match bmi.category {
        BMICategory::Underweight => {
            println!("You are underweight.")
        }
        BMICategory::Normal => {
            println!("You are a normal weight.")
        }
        BMICategory::Overweight => {
            println!("You are overweight.")
        }
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
