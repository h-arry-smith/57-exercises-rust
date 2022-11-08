use std::io;
use std::io::Write;

struct CreditCardDebt {
    daily_rate: f64,
    balance: f64,
    monthly_payment: f64,
}

impl CreditCardDebt {
    fn new(daily_rate: f64, balance: f64, monthly_payment: f64) -> Self {
        CreditCardDebt {
            daily_rate: daily_rate / 100.0 / 365.0,
            balance,
            monthly_payment,
        }
    }

    fn build() -> Self {
        Self::new(
            get_number("What is the APR on the card (%)? "),
            get_number("What is the balance? "),
            get_number("What is the monthly payment you can make? "),
        )
    }

    fn months_to_pay_off(&self) -> f64 {
        let constant = -(1.0 / 30.0);
        let top_half = f64::log10(
            1.0 + (self.balance / self.monthly_payment)
                * (1.0 - f64::powf(1.0 + self.daily_rate, 30.0)),
        );
        let bottom_half = f64::log10(1.0 + self.daily_rate);

        (constant * (top_half / bottom_half)).ceil()
    }
}

fn main() {
    let debt = CreditCardDebt::build();

    println!(
        "It will take you {} months to pay off this card.",
        debt.months_to_pay_off()
    );
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
