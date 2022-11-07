use std::io;
use std::io::Write;

fn main() {
    let principal = get_number("What is the principal amount? ");

    let rate: f64 = get_number("What is the rate? ");
    let rate = rate / 100.0;

    let years: f64 = get_number("What is the number of years? ");
    let compounds_per_year: f64 =
        get_number("What is the number of times the interest is compounded per year? ");

    let final_amount = complex_interest(&principal, &rate, &years, &compounds_per_year);

    println!("${:.2} invested at {}% for {years} years compounded {compounds_per_year} times per year is ${:.2}", 
    principal, rate * 100.0, final_amount);
}

fn complex_interest(principal: &f64, rate: &f64, years: &f64, compounds_per_year: &f64) -> f64 {
    principal * (1.0 + (rate / compounds_per_year)).powf(years * compounds_per_year)
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
