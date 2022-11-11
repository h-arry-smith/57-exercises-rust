use std::cmp::Ordering;
use std::io;
use std::io::Write;
use std::str::FromStr;

use rand::Rng;

fn main() {
    println!("Let's play Guess the Number.");
    loop {
        let difficulty = get_number_between("Pick a difficulty level (1, 2, or 3): ", 1, 3);
        let target_number = get_random_number(difficulty);
        let mut guess_count = 0;

        loop {
            let guess = get_number("What's your guess? ");

            guess_count += 1;

            if guess.is_err() {
                continue;
            }

            let guess: i32 = guess.unwrap();

            match guess.cmp(&target_number) {
                Ordering::Less => println!("Too low."),
                Ordering::Greater => println!("Too high."),
                Ordering::Equal => break,
            }
        }

        println!("You got it in {} guesses", guess_count);

        let play_again = get_bool("Would you like to play again?");

        if !play_again {
            break;
        }
    }
}

fn get_bool(question_text: &str) -> bool {
    let mut answer = String::new();

    loop {
        print!("{question_text} (y/n) ");
        io::stdout().flush().unwrap();

        io::stdin()
            .read_line(&mut answer)
            .expect("Could not read a line.");

        match answer.to_lowercase().trim() {
            "y" => return true,
            "n" => return false,
            _ => continue,
        }
    }
}

fn get_random_number(difficulty: i32) -> i32 {
    let mut rng = rand::thread_rng();

    match difficulty {
        1 => rng.gen_range(1..=10),
        2 => rng.gen_range(1..=100),
        3 => rng.gen_range(1..=1000),
        _ => rng.gen_range(1..=10),
    }
}

fn get_number<T: FromStr>(question_text: &str) -> Result<T, T::Err> {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    number.trim().parse()
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
