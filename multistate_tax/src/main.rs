use std::io;
use std::io::Write;

#[derive(Debug)]
struct StateTax {
    name: String,
    tax: f64,
    additional_taxes: Vec<StateTax>,
}

impl StateTax {
    fn new(name: &str, tax: f64, additional_taxes: Vec<StateTax>) -> Self {
        StateTax {
            name: name.to_string(),
            tax,
            additional_taxes,
        }
    }

    fn has_additional_taxes(&self) -> bool {
        self.additional_taxes.len() > 0
    }

    fn additional_taxes(&self) -> &Vec<StateTax> {
        &self.additional_taxes
    }

    fn tax(&self, amount: &f64) -> f64 {
        amount * (self.tax / 100.0)
    }
}

fn main() {
    let states = vec![
        StateTax::new("Illinois", 8.0, vec![]),
        StateTax::new(
            "Wisconsin",
            5.0,
            vec![
                StateTax::new("Eau Claire", 5.5, vec![]),
                StateTax::new("Dunn", 5.4, vec![]),
                StateTax::new("Other", 5.0, vec![]),
            ],
        ),
        StateTax::new("Other", 0.0, vec![]),
    ];

    let mut state = get_state(&states);

    while state.has_additional_taxes() {
        state = get_state(&state.additional_taxes());
    }

    let order_amount = get_number("What is the order amount? ");
    let tax = state.tax(&order_amount);

    println!("The tax is ${:.2}", tax);
    println!("The total is ${:.2}", order_amount + tax);
}

fn get_state(states: &Vec<StateTax>) -> &StateTax {
    println!("Select from the below options: ");
    for (i, state) in states.iter().enumerate() {
        println!("{}) {}", i + 1, state.name);
    }

    loop {
        let chosen_index: usize = get_number("> ");
        if chosen_index <= 0 {
            continue;
        }

        let chosen_index = chosen_index - 1;

        match states.get(chosen_index) {
            Some(state) => return state,
            None => {
                continue;
            }
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
