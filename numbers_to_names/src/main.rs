use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let months = [
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December",
    ];

    let chosen_month = get_number_between("Please enter the number of the month: ", 1, 12) - 1;
    println!(
        "The name of the month is {}",
        months.get(chosen_month).unwrap()
    );
}

fn get_number_between<T>(question_text: &str, min: T, max: T) -> T
where
    T: FromStr + PartialOrd,
{
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

        if number >= min && number <= max {
            return number;
        }
    }
}
