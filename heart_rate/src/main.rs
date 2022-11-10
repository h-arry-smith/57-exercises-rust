use std::io;
use std::io::Write;

struct Person {
    resting_pulse: i32,
    age: i32,
}

struct IntensityBPM {
    intensity: i32,
    bpm: f64,
}

impl IntensityBPM {
    fn new(intensity: i32, bpm: f64) -> Self {
        IntensityBPM { intensity, bpm }
    }
}

struct KarvonenHeartRates {
    heart_rates: Vec<IntensityBPM>,
}

impl KarvonenHeartRates {
    fn new(person: &Person) -> Self {
        let mut heart_rates = Vec::new();

        for percent in (55..=95).step_by(5) {
            heart_rates.push(IntensityBPM::new(
                percent,
                karvonen_heart_rate(person, percent as f64 / 100.0),
            ));
        }

        KarvonenHeartRates { heart_rates }
    }
}

impl IntoIterator for KarvonenHeartRates {
    type Item = IntensityBPM;

    type IntoIter = <Vec<IntensityBPM> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.heart_rates.into_iter()
    }
}

fn main() {
    let pulse = get_number("Resting Pulse? ");
    let age = get_number("Age? ");

    let person = Person {
        resting_pulse: pulse,
        age,
    };

    println!(
        "Resting Pulse: {}     Age: {}",
        person.resting_pulse, person.age
    );

    let rates = KarvonenHeartRates::new(&person);

    println!("|{:^15}|{:^15}|", "Intensity (%)", "BPM");
    println!("{}", "-".to_string().repeat(33));
    for column in rates.into_iter() {
        println!("|{:^15}|{:^15.1}|", column.intensity, column.bpm);
    }
}

fn get_number<T: std::str::FromStr>(question_text: &str) -> T {
    loop {
        print!("{question_text}");
        io::stdout().flush().unwrap();

        let mut number = String::new();

        io::stdin()
            .read_line(&mut number)
            .expect("Failed to read line.");

        let number: T = match number.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        return number;
    }
}
fn karvonen_heart_rate(person: &Person, intensity: f64) -> f64 {
    (((220.0 - person.age as f64) - person.resting_pulse as f64) * intensity)
        + person.resting_pulse as f64
}
