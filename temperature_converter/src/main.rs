use std::io;
use std::io::Write;
use std::{collections::HashMap, fmt::Display};

struct Farenheit(f64);

impl Display for Farenheit {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°F", self.0)
    }
}

impl From<Celsius> for Farenheit {
    fn from(celsius: Celsius) -> Self {
        Farenheit((celsius.0 * 9.0 / 5.0) + 32.0)
    }
}

struct Celsius(f64);

impl Display for Celsius {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:.2}°C", self.0)
    }
}

impl From<Farenheit> for Celsius {
    fn from(farenheit: Farenheit) -> Self {
        Celsius((farenheit.0 - 32.0) * 5.0 / 9.0)
    }
}

enum ConversionOption {
    FarenheitToCelsius,
    CelsiusToFarenheit,
}

impl Display for ConversionOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ConversionOption::FarenheitToCelsius => write!(f, "Farenheit to Celsius"),
            ConversionOption::CelsiusToFarenheit => write!(f, "Celsius to Farenheit"),
        }
    }
}

fn main() {
    let conversion_options = HashMap::from([
        (String::from("C"), ConversionOption::FarenheitToCelsius),
        (String::from("F"), ConversionOption::CelsiusToFarenheit),
    ]);

    let chosen_option = get_option("Your choice: ", &conversion_options);

    match chosen_option {
        ConversionOption::FarenheitToCelsius => {
            let farenheit = Farenheit(get_number("Temperature in °F: "));
            println!("The temperature in °C is {}", Celsius::from(farenheit));
        }
        ConversionOption::CelsiusToFarenheit => {
            let celsius = Celsius(get_number("Temperature in °C: "));
            println!("The temperature in °F is {}", Farenheit::from(celsius));
        }
    }
}

fn get_option<'a, T: Display>(question_text: &str, options: &'a HashMap<String, T>) -> &'a T {
    let mut choice = String::new();

    for (name, option) in options {
        println!("'{}' : {}", name, option)
    }

    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        return match options.get(choice.trim()) {
            Some(option) => option,
            None => continue,
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
