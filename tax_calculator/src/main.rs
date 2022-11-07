use std::io;
use std::io::Write;

fn main() {
    let order_amount = get_number("What is the order amount? ");
    let state = get_state();

    let tax = calculate_tax(order_amount, state);

    println!("The subtotal is ${:.2}", order_amount);

    let total = match tax {
        Some(tax) => {
            println!("The tax is ${:.2}", tax);
            order_amount + tax
        }
        None => order_amount,
    };

    println!("The total is ${:.2}", total);
}

enum StateTax {
    Wisconsin,
    None,
}

fn calculate_tax(order_amount: f64, state: StateTax) -> Option<f64> {
    match state {
        StateTax::Wisconsin => Some(order_amount * (5.5 / 100.0)),
        StateTax::None => None,
    }
}

fn get_state() -> StateTax {
    print!("What is the state? ");
    io::stdout().flush().unwrap();

    let mut state = String::new();

    io::stdin()
        .read_line(&mut state)
        .expect("Failed to read line.");

    state = state.trim().to_lowercase();

    match state.as_str() {
        "wi" => StateTax::Wisconsin,
        _ => StateTax::None,
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
