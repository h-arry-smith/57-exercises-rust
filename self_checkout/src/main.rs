use std::fmt;
use std::io;
use std::io::Write;
use std::ops::Add;
use std::ops::Mul;

const TAX_RATE: f64 = 5.5 / 100.0;

#[derive(Clone, Copy)]
struct Money {
    cents: i32,
}

impl Money {
    fn new(cents: i32) -> Self {
        Money { cents }
    }
}

impl fmt::Display for Money {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "${:.2}", self.cents as f64 / 100.0)
    }
}

impl Add for Money {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self {
            cents: self.cents + rhs.cents,
        }
    }
}

impl Mul for Money {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self {
            cents: self.cents * rhs.cents,
        }
    }
}

impl Mul<i32> for Money {
    type Output = Self;

    fn mul(self, rhs: i32) -> Self::Output {
        let result = self.cents * rhs;

        Self { cents: result }
    }
}

impl Mul<f64> for Money {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        let cents = self.cents as f64;
        let cents = cents * rhs;
        let cents = cents.floor();
        let cents = cents as i32;

        Self { cents }
    }
}

struct LineItem {
    quantity: i32,
    price: Money,
}

struct Basket {
    items: Vec<LineItem>,
}

impl LineItem {
    fn new(quantity: i32, price: Money) -> Self {
        Self { quantity, price }
    }

    fn total(&self) -> Money {
        self.price * self.quantity
    }
}

impl Basket {
    fn new() -> Self {
        Self { items: Vec::new() }
    }

    fn count(&self) -> usize {
        self.items.len()
    }

    fn add_line_item(&mut self, line_item: LineItem) {
        self.items.push(line_item);
    }

    fn subtotal(&self) -> Money {
        let total = Money { cents: 0 };

        self.items
            .iter()
            .fold(total, |accum, item| accum + item.total())
    }

    fn tax(&self) -> Money {
        self.subtotal() * TAX_RATE
    }

    fn total(&self) -> Money {
        self.subtotal() + self.tax()
    }
}

fn main() {
    let mut basket = Basket::new();

    loop {
        let item_count = basket.count() + 1;

        let price =
            get_number_or_nothing(&format!("Enter the price for item {item_count} (cents)? "));

        let price = match price {
            Some(price) => Money::new(price),
            None => break,
        };

        let quantity = get_number(&format!("Enter the quantity for item {item_count}? "));

        basket.add_line_item(LineItem::new(quantity, price));
    }

    println!("Subtotal: {}", basket.subtotal());
    println!("Tax: {}", basket.tax());
    println!("Toal: {}", basket.total());
}

fn get_number_or_nothing(question_text: &str) -> Option<i32> {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut number = String::new();

    io::stdin()
        .read_line(&mut number)
        .expect("Failed to read line.");

    match number.trim().parse() {
        Ok(num) => Some(num),
        Err(_) => None,
    }
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
