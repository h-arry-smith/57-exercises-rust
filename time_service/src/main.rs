use chrono::DateTime;
use chrono::Utc;
use serde::Serialize;
use std::time::SystemTime;
use warp::Filter;

#[derive(Serialize)]
struct Time {
    current_time: String,
}

impl Time {
    fn new() -> Self {
        let now = SystemTime::now();
        let now: DateTime<Utc> = now.into();
        let now = now.to_rfc3339();

        Time { current_time: now }
    }
}

#[tokio::main]
async fn main() {
    // GET /hello/warp => 200 OK with body "Hello, warp!"
    let hello = warp::path!("time").map(|| {
        let time = Time::new();
        warp::reply::json(&time)
    });

    warp::serve(hello).run(([127, 0, 0, 1], 3030)).await;
}
