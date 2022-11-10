use std::{
    io::{self, Write},
    str::FromStr,
};

fn main() {
    let mut numbers: Vec<f64> = Vec::new();
    get_vector(&mut numbers);

    let total = numbers.iter().sum::<f64>();

    println!("The total is {total}.");
}

fn get_vector<T: FromStr>(vector: &mut Vec<T>) {
    loop {
        print!("Enter number {}: ", vector.len() + 1);
        io::stdout().flush().unwrap();

        match get_number() {
            Ok(n) => vector.push(n),
            Err(_) => break,
        }
    }
}

fn get_number<T: std::str::FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    return number.trim().parse();
}
