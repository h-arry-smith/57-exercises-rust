use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Astronaut {
    name: String,
    craft: String,
}

#[derive(Debug, Deserialize)]
struct Astronauts {
    // message: String,
    people: Vec<Astronaut>,
}

fn main() -> Result<(), Error> {
    let astronauts: Astronauts =
        reqwest::blocking::get("http://api.open-notify.org/astros.json")?.json()?;

    println!(
        "There are {} people in space right now:\n\n",
        astronauts.people.len()
    );

    println!("{:<20} | {:<15}", "Name", "Craft");
    println!("{}", "-".repeat(38));

    for astronaut in astronauts.people {
        println!("{:<20} | {:<15}", astronaut.name, astronaut.craft);
    }

    Ok(())
}
