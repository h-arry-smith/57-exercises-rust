use std::io::Write;
use std::{fmt::Display, fs, io};

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
struct ProductRecord {
    name: String,
    price: f64,
    quantity: i32,
}

impl Display for ProductRecord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Name: {}\nPrice: ${:.2}\nQuantity: {}",
            self.name, self.price, self.quantity
        )
    }
}

fn main() {
    let data = fs::read_to_string("products.json").expect("Could not open file.");
    let data: Value = serde_json::from_str(data.as_str()).unwrap();

    let products: Vec<ProductRecord> = serde_json::from_str(&data["products"].to_string()).unwrap();

    loop {
        let name = get_string("What is the product name? ");

        let product = products.iter().find(|p| p.name == name);

        match product {
            Some(product) => {
                println!("{}", product);
                break;
            }
            None => {
                println!("Sorry, that product was not found in our inventory.");
            }
        }
    }
}

fn get_string(question_text: &str) -> String {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut answer = String::new();

    io::stdin()
        .read_line(&mut answer)
        .expect("Could not read a line.");

    answer.trim().to_string()
}
