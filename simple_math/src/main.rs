use std::io;
use std::io::Write;

fn main() {
    let a = get_number("What is the first number? ");
    let b = get_number("What is the second number? ");

    let add_result = a + b;
    let sub_result = a - b;
    let mul_result = a * b;
    let div_result = a / b;

    println!("{a} + {b} = {add_result}");
    println!("{a} - {b} = {sub_result}");
    println!("{a} * {b} = {mul_result}");
    println!("{a} / {b} = {div_result}");
}

fn get_number(question_text: &str) -> i32 {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: i32 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
