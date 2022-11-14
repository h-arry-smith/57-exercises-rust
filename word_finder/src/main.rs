use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 3 {
        println!("Usage: wordfinder [input] [output]");
        process::exit(0);
    }

    let input = &args[1];
    let output = &args[2];

    let input = fs::read_to_string(input).expect("Could not open file: {input}");
    let input = input.replace("utilize", "use");

    fs::write(output, input).expect("Could not write to file: {output}");
}
