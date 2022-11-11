use std::io;
use std::io::Write;
use std::str::FromStr;

fn main() {
    let mut numbers: Vec<f64> = vec![];
    get_vector(&mut numbers);

    let numbers_as_strings: Vec<String> = numbers.iter().map(|x| x.to_string()).collect();

    println!("Numbers: {}", numbers_as_strings.join(", "));
    println!("The average is {}", average(&numbers));
    println!("The minimum is {}", minimum(&numbers));
    println!("The maximum is {}", maximum(&numbers));
    println!("The standard deviation is {}", standard_deviation(&numbers));
}

fn average(numbers: &Vec<f64>) -> f64 {
    numbers.iter().sum::<f64>() / numbers.len() as f64
}

fn minimum(numbers: &Vec<f64>) -> f64 {
    let mut numbers = numbers.clone();
    numbers.push(f64::NAN);

    numbers.into_iter().reduce(f64::min).unwrap()
}

fn maximum(numbers: &Vec<f64>) -> f64 {
    let mut numbers = numbers.clone();
    numbers.push(f64::NAN);

    numbers.into_iter().reduce(f64::max).unwrap()
}

fn standard_deviation(numbers: &Vec<f64>) -> f64 {
    let numbers = numbers.clone();
    let mean = average(&numbers);

    let squares = numbers.iter().map(|x| (x - mean).powf(2.0)).collect();

    let squares_avg = average(&squares);

    f64::sqrt(squares_avg)
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

    return number.trim().parse();
}
