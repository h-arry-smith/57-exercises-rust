use std::io;
use std::io::Write;

struct ExchangeRate {
    name: String,
    rate: f64,
}

impl ExchangeRate {
    fn new(name: &str, rate: f64) -> Self {
        ExchangeRate {
            name: name.to_string(),
            rate,
        }
    }
}

fn main() {
    let rates = Vec::from([
        ExchangeRate::new("USD", 137.51),
        ExchangeRate::new("GBP", 87.68),
        ExchangeRate::new("YEN", 14521.25),
    ]);

    let euros = get_number("How many euros are you exchanging? ");
    let exchange_rate = get_exchange_rate("Choose an exchange rate? ", &rates);

    let new_amount = exchange(euros, exchange_rate.rate);

    println!(
        "{euros}€ at an exchange rate of {:.2} is {:.2} {}",
        exchange_rate.rate, new_amount, exchange_rate.name
    );
}

fn exchange(euros: f64, exchange_rate: f64) -> f64 {
    (euros * exchange_rate) / 100.0
}

fn get_exchange_rate<'a>(question_text: &str, vector: &'a Vec<ExchangeRate>) -> &'a ExchangeRate {
    println!("{question_text}");

    for (i, option) in vector.iter().enumerate() {
        println!("{}) {}", i + 1, option.name);
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();

        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice: usize = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return match vector.get(choice - 1) {
            Some(chosen_item) => chosen_item,
            None => continue,
        };
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
