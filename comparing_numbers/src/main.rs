use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<f64> = Vec::new();

    get_vector(&mut numbers);

    match largest(&numbers) {
        Some(n) => println!("The largest number is {}", n),
        None => println!("There is no largest number!"),
    };
}

fn largest<T: PartialOrd>(vector: &Vec<T>) -> Option<&T> {
    let largest = vector.first();

    if largest.is_none() {
        return largest;
    }

    let mut largest = largest.unwrap();

    for value in vector {
        if value > largest {
            largest = value
        }
    }

    Some(largest)
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

fn get_number<T: FromStr>() -> Result<T, <T as FromStr>::Err> {
    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    number.trim().parse()
}
