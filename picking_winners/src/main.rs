use rand::Rng;
use std::io;
use std::io::Write;

fn main() {
    let mut names = vec![];
    get_names(&mut names);

    let mut rng = rand::thread_rng();

    let name = &names[rng.gen_range(0..names.len())];

    println!("The winner is... {}", name);
}

fn get_names(vector: &mut Vec<String>) {
    loop {
        print!("Enter a name: ");
        io::stdout().flush().unwrap();

        let answer = get_string();

        if answer.len() == 0 {
            break;
        } else {
            vector.push(answer);
        }
    }
}

fn get_string() -> String {
    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Failed to read line.");

    answer.trim().to_string()
}
