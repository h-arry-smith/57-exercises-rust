use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader},
    process,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("Usage: wordfreq [input]");
        process::exit(0);
    }

    let f = File::open(&args[1]).expect("Could not open file.");
    let file = BufReader::new(f);

    let mut words: HashMap<String, usize> = HashMap::new();

    for line in file.lines() {
        if let Ok(line) = line {
            let mut line = line;

            let punctuation = [
                "?", "!", ",", ";", ":", "-", "(", ")", "[", "]", "{", "}", "\"",
            ];

            for mark in punctuation {
                line = line.replace(mark, " ");
            }

            for word in line.split(" ") {
                let word = word.trim().to_string();

                if word.len() > 0 {
                    match words.get_mut(&word) {
                        Some(count) => {
                            *count += 1;
                        }
                        None => {
                            words.insert(word, 1);
                        }
                    }
                }
            }
        }
    }

    let mut frequencies: Vec<(String, usize)> = words
        .into_iter()
        .map(|(word, frequency)| (word, frequency))
        .collect();

    frequencies.sort_by(|a, b| b.1.cmp(&a.1));

    let max_frequency = frequencies.iter().take(30).map(|w| w.1).max().unwrap();

    let max_word_length = frequencies
        .iter()
        .take(30)
        .map(|w| w.0.len())
        .max()
        .unwrap();

    for (word, frequency) in frequencies.iter().take(30) {
        let percentage = (*frequency as f64 / max_frequency as f64) * 50.0;
        let percentage = percentage.round();

        let bar_chart = format!("{:<50}", "█".repeat(percentage as usize));

        let word = format!("{}{}", " ".repeat(max_word_length - word.len()), word);

        println!("{} - {:<5} {}", word, frequency, bar_chart);
    }
}
