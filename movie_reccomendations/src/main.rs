use std::env;
use std::io;
use std::io::Write;
use std::process;

use reqwest::Error;

use serde::Deserialize;
use urlencoding::encode;

#[derive(Debug, Deserialize)]
struct Movie {
    overview: String,
    release_date: Option<String>,
    original_title: String,
    vote_average: f64,
}

#[derive(Debug, Deserialize)]
struct MovieDB {
    results: Vec<Movie>,
}

fn main() -> Result<(), Error> {
    let api_key = match env::var("API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("No API_KEY set");
            process::exit(0);
        }
    };

    let movie_name = get_string("Enter a movie: ");
    let movie_name = encode(&movie_name);

    let url = format!(
        "https://api.themoviedb.org/3/search/movie?api_key={}&language=en-US&query={}&page=1&include_adult=false",
        api_key,
        movie_name
    );

    let moviedb: MovieDB = reqwest::blocking::get(url)?.json()?;

    for movie in moviedb.results {
        println!("Title: {}", movie.original_title);
        if let Some(date) = movie.release_date {
            println!("Release Date: {}", date);
        }
        println!("Rating: {:.1} / 10", movie.vote_average);
        println!("");
    }

    Ok(())
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
