use rand::prelude::SliceRandom;
use std::io::{self, Write};

use rand::Rng;

struct PasswordGenerator {
    length: usize,
    special_characters: usize,
    numbers: usize,
}

impl PasswordGenerator {
    fn new(length: usize, special_characters: usize, numbers: usize) -> PasswordGenerator {
        assert!(special_characters + numbers <= length);

        PasswordGenerator {
            length,
            special_characters,
            numbers,
        }
    }

    fn generate(&self) -> String {
        let mut password_characters: Vec<char> = vec![];
        let mut rng = rand::thread_rng();

        for _ in 0..self.special_characters {
            password_characters.push(*random_from_array(&['#', '$', '%', '&', '*', '!', '@']));
        }

        for _ in 0..self.numbers {
            password_characters.push(char::from(rng.gen_range(48..58)));
        }

        for _ in 0..self.letters_to_add() {
            if rng.gen_bool(0.5) {
                password_characters.push(char::from(rng.gen_range(65..91)));
            } else {
                password_characters.push(char::from(rng.gen_range(97..123)));
            }
        }

        password_characters.shuffle(&mut rng);
        password_characters.iter().collect()
    }

    fn letters_to_add(&self) -> usize {
        self.length - self.special_characters - self.numbers
    }
}

impl Iterator for PasswordGenerator {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.generate())
    }
}

fn random_from_array<T>(array: &[T]) -> &T {
    let mut rng = rand::thread_rng();
    let index = rng.gen_range(0..array.len());
    &array[index]
}

fn main() {
    let length: usize = get_number("Enter the length? ");
    let special_characters: usize = get_number("How many special characters? ");
    let numbers: usize = get_number("How many numbers? ");

    let password_gen = PasswordGenerator::new(length, special_characters, numbers);

    for password in password_gen.take(10) {
        println!("{}", password);
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
