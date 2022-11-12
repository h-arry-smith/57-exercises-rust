use std::io::{self, Write};

fn main() {
    let numbers = get_input();
    let even_numbers = filter(&numbers, |&x| x % 2 == 0);

    let even_numbers_str = even_numbers
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    println!("The even numbers are: {}", even_numbers_str);
}

fn filter<I, F>(list: &Vec<I>, f: F) -> Vec<I>
where
    I: Copy,
    F: Fn(&I) -> bool,
{
    let mut new_list = vec![];
    for i in 0..list.len() {
        if f(&list[i]) {
            new_list.push(list[i]);
        }
    }

    new_list
}

fn get_input() -> Vec<i32> {
    print!("Enter a list of numbers, seperated by spaces: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Can't read a line.");

    let input = input.trim();

    let input = input.split(' ').filter_map(|s| s.parse().ok());

    input.collect()
}
