use std::io;
use std::io::Write;

fn main() {
    let people = get_number("How many people? ");
    let pizzas = get_number("How many pizzas? ");
    let slices_per_pizza = get_number("How many slices does each pizza have? ");

    let total_slices = pizzas * slices_per_pizza;
    let slices_each = total_slices / people;
    let remaining_slices = total_slices % people;

    println!(
        "{} with {}",
        pluralize(people, "person", Some("people")),
        pluralize(pizzas, "pizza", None)
    );
    println!(
        "Each person gets {} each",
        pluralize(slices_each, "piece", None)
    );
    println!(
        "There are {} leftover",
        pluralize(remaining_slices, "piece", None)
    );
}

fn pluralize(count: i32, singular: &str, plural: Option<&str>) -> String {
    let plural: String = match plural {
        Some(plural) => String::from(plural),
        None => String::from(format!("{singular}s")),
    };

    if count == 1 {
        return format!("{count} {singular}");
    }

    format!("{count} {plural}")
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
