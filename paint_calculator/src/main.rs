use std::collections::HashMap;
use std::f64::consts::PI;
use std::io;
use std::io::Write;

const SQFT_PER_GALLON: f64 = 350.0;

enum RoomOption {
    Rectangle,
    Circle,
    LShaped,
}

pub trait Room {
    fn ceiling_sqft(&self) -> f64;
}

struct RectangularRoom {
    width: f64,
    length: f64,
}

struct CircularRoom {
    radius: f64,
}

struct LShapedRoom {
    rect1: RectangularRoom,
    rect2: RectangularRoom,
}

impl RectangularRoom {
    fn from_user() -> Self {
        let width = get_number("Width of room? ");
        let length = get_number("Length of room? ");

        RectangularRoom { width, length }
    }
}

impl Room for RectangularRoom {
    fn ceiling_sqft(&self) -> f64 {
        self.width * self.length
    }
}

impl CircularRoom {
    fn from_user() -> Self {
        let radius = get_number("Radius of room? ");

        CircularRoom { radius }
    }
}

impl Room for CircularRoom {
    fn ceiling_sqft(&self) -> f64 {
        self.radius * self.radius * PI
    }
}

impl LShapedRoom {
    fn from_user() -> Self {
        println!("Rectangle One");
        let rect1 = RectangularRoom::from_user();

        println!("Rectangle Two");
        let rect2 = RectangularRoom::from_user();

        LShapedRoom { rect1, rect2 }
    }
}

impl Room for LShapedRoom {
    fn ceiling_sqft(&self) -> f64 {
        self.rect1.ceiling_sqft() + self.rect2.ceiling_sqft()
    }
}

fn main() {
    let room_options = HashMap::from([
        (String::from("rect"), RoomOption::Rectangle),
        (String::from("circle"), RoomOption::Circle),
        (String::from("lshaped"), RoomOption::LShaped),
    ]);

    let choice = get_option("What room type are you calculating? ", &room_options);

    let room: Box<dyn Room>;

    match choice {
        RoomOption::Rectangle => {
            room = Box::new(RectangularRoom::from_user());
        }
        RoomOption::Circle => {
            room = Box::new(CircularRoom::from_user());
        }
        RoomOption::LShaped => {
            room = Box::new(LShapedRoom::from_user());
        }
    }

    let sq_feet = room.ceiling_sqft();
    let gallons_needed = gallons_to_cover(sq_feet);

    println!(
        "You will need to purchase {gallons_needed} gallons of paint to cover {} feet",
        sq_feet.ceil()
    );
}

fn gallons_to_cover(sq_feet: f64) -> f64 {
    (sq_feet / SQFT_PER_GALLON).ceil()
}

fn get_option<'a, T>(question_text: &str, options: &'a HashMap<String, T>) -> &'a T {
    println!("{question_text}");

    for option in options.keys() {
        println!(" - {option} ?")
    }

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut choice = String::new();
        io::stdin()
            .read_line(&mut choice)
            .expect("Failed to read line.");

        let choice = choice.trim();

        let result: &T = match options.get(choice) {
            Some(option) => option,
            None => continue,
        };

        return result;
    }
}

fn get_number(question_text: &str) -> f64 {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: f64 = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
