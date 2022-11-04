use std::collections::HashMap;
use std::io;
use std::io::Write;

const CONVERSION_FACTOR: f64 = 0.09290304;

struct Room {
    length: f64,
    width: f64,
}

#[derive(Debug)]
enum Unit {
    SquareFeet,
    SquareMeter,
}

impl Room {
    fn new(length: f64, width: f64) -> Self {
        Self { length, width }
    }

    fn area(&self) -> f64 {
        self.length * self.width
    }
}

fn main() {
    let mut unit_options = HashMap::new();
    unit_options.insert(String::from("sqft"), Unit::SquareFeet);
    unit_options.insert(String::from("sqm"), Unit::SquareMeter);

    let option = get_option("What unit are you working with?", &unit_options);

    let unit = match option {
        Unit::SquareFeet => String::from("feet"),
        Unit::SquareMeter => String::from("meters"),
    };

    let length = get_number(&format!("What is the length of the room in {unit}? "));
    let width = get_number(&format!("What is the width of the room in {unit}? "));

    let room = Room::new(length, width);

    let sq_feet;
    let sq_meter;

    match option {
        Unit::SquareFeet => {
            sq_feet = room.area();
            sq_meter = to_sqm(room.area());
        }
        Unit::SquareMeter => {
            sq_meter = room.area();
            sq_feet = to_sqft(room.area());
        }
    };

    println!("You entered dimensions of {length} feet by {width} feet.");
    println!("The area is");
    println!("{} square feet", sq_feet);
    println!("{:.3} square meters", sq_meter);
}

fn to_sqm(sq_feet: f64) -> f64 {
    sq_feet * CONVERSION_FACTOR
}

fn to_sqft(sq_meter: f64) -> f64 {
    sq_meter / CONVERSION_FACTOR
}

fn get_option<'a>(question_text: &str, options: &'a HashMap<String, Unit>) -> &'a Unit {
    println!("{question_text}");

    for option in options.keys() {
        println!(" - {option} ?")
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice = choice.trim();

        let result: &Unit = match options.get(choice) {
            Some(unit) => unit,
            None => continue,
        };

        return result;
    }
}

fn get_number(question_text: &str) -> f64 {
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
