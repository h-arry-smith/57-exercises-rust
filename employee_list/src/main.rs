use std::io::prelude::*;
use std::{fs::File, io};

fn display_employees(employees: &Vec<String>) {
    println!("There are {} employees:", employees.len());
    for employee in employees {
        println!("{}", employee);
    }
}

fn main() -> std::io::Result<()> {
    let mut employees = vec![];
    let file = File::open("employee_list.txt")?;
    let file = io::BufReader::new(file);

    for line in file.lines() {
        employees.push(line.unwrap().trim().to_string())
    }

    employees.sort();

    display_employees(&employees);

    let employee_name = get_string("Enter an employee name to remove: ")
        .trim()
        .to_string();

    match employees.binary_search(&employee_name) {
        Ok(index) => {
            employees.remove(index);
        }
        Err(_) => {
            println!("Error: No employee with name {employee_name}!");
        }
    }

    display_employees(&employees);

    let mut file = File::create("employee_list.txt")?;

    for employee in &employees {
        file.write(format!("{}\n", employee).as_bytes())?;
    }

    Ok(())
}

fn get_string(question_text: &str) -> String {
    let mut answer = String::new();

    print!("{question_text}");
    io::stdout().flush().unwrap();

    io::stdin()
        .read_line(&mut answer)
        .expect("Can't read a line");

    answer.trim().to_string()
}
