use chrono::NaiveDate;
use std::{
    fs::File,
    io::{BufRead, BufReader},
};
use tabular::row;
use tabular::Table;

#[derive(Debug)]
struct Employee {
    first_name: String,
    second_name: String,
    role: String,
    seperation_date: Option<NaiveDate>,
}

impl Employee {
    fn new(parts: Vec<&str>) -> Employee {
        let seperation_date = match parts.get(3) {
            Some(string_date) => Some(NaiveDate::parse_from_str(string_date, "%Y-%m-%d").unwrap()),
            None => None,
        };

        Employee {
            first_name: parts.get(0).unwrap().to_string(),
            second_name: parts.get(1).unwrap().to_string(),
            role: parts.get(2).unwrap().to_string(),
            seperation_date,
        }
    }

    fn name(&self) -> String {
        format!("{} {}", self.first_name, self.second_name)
    }
}

fn main() {
    let file = File::open("employees.txt").expect("Could not open file.");
    let file = BufReader::new(file);

    let mut employees = vec![];

    for line in file.lines() {
        if let Ok(line) = line {
            let parts = line.split(",").map(|x| x.trim()).collect::<Vec<&str>>();

            employees.push(Employee::new(parts));
        }
    }

    employees.sort_by(|a, b| a.second_name.cmp(&b.second_name));

    let mut table = Table::new("{:<} {:<} {:<}");
    table.add_row(row!("Name", "Position", "Seperation Date"));

    for employee in employees {
        let date = match employee.seperation_date {
            Some(date) => date.format("%Y-%m-%d").to_string(),
            None => String::from(""),
        };

        table.add_row(row!(employee.name(), employee.role, date));
    }

    print!("{}", table);
}
