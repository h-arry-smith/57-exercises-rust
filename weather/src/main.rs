use reqwest::Error;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct CurrentWeather {
    temperature: f64,
    windspeed: f64,
    winddirection: f64,
    weathercode: f64,
}

#[derive(Debug, Deserialize)]
struct Weather {
    current_weather: CurrentWeather,
}

fn main() -> Result<(), Error> {
    let url = "https://api.open-meteo.com/v1/forecast?latitude=51.5002&longitude=-0.1262&current_weather=true";

    let weather: Weather = reqwest::blocking::get(url)?.json()?;

    println!("The current weather in London\n");

    println!("Temperature: {}ºC", weather.current_weather.temperature);
    println!(
        "Wind: {}m/s {}º",
        weather.current_weather.windspeed, weather.current_weather.winddirection,
    );

    println!(
        "Weather: {}",
        get_code(weather.current_weather.weathercode as i32)
    );

    Ok(())
}

fn get_code(weather_code: i32) -> &'static str {
    match weather_code {
        0..=3 => "Clear Sky",
        4..=9 => "Haze",
        10 => "Mist",
        11..=12 => "Fog",
        13 => "Lightning",
        14..=16 => "Sea Rain",
        17 => "Thunderstorm",
        18 => "Squall",
        19 => "Funnel Cloud",
        20..=29 => "Rain",
        30..=39 => "Duststorm",
        40..=49 => "Fog",
        50..=59 => "Drizzle",
        60..=69 => "Rain",
        70..=79 => "Snow",
        80..=99 => "Showers",
        _ => "Unknown",
    }
}
