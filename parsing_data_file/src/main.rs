use std::{
    fmt::Display,
    fs::File,
    io::{BufRead, BufReader},
};

struct Record {
    last: String,
    first: String,
    salary: Money,
}

struct Money {
    value: i32,
}

impl Money {
    fn new(value: i32) -> Self {
        Money { value }
    }
}

impl Display for Money {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "${:.2}", self.value as f64 / 100.0)
    }
}

fn main() {
    let f = File::open("data.csv").expect("Could not open file.");
    let file = BufReader::new(f);

    let mut records: Vec<Record> = vec![];

    for line in file.lines() {
        if let Ok(line) = line {
            let parts: Vec<&str> = line.trim().split(",").collect();

            records.push(Record {
                last: parts[0].to_string(),
                first: parts[1].to_string(),
                salary: Money::new(parts[2].parse::<i32>().unwrap() * 100),
            })
        }
    }

    println!("{:<10} {:<10} {:<10}", "Last", "First", "Salary");
    println!("{}", "-".repeat(31));
    for record in records {
        println!(
            "{:<10} {:<10} {:<10}",
            record.last, record.first, record.salary
        );
    }
}
