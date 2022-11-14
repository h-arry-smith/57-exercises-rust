use std::io::BufRead;
use std::{fs::File, io::BufReader};

fn main() {
    let f = File::open("names.txt").expect("Could not open file.");
    let file = BufReader::new(f);

    let mut names = vec![];

    for line in file.lines() {
        if let Ok(line) = line {
            names.push(line.trim().to_string());
        }
    }

    names.sort_unstable();

    println!("Ther are {} names", names.len());
    println!("{}", "-".repeat(7));

    for name in names {
        println!("{}", name);
    }
}
