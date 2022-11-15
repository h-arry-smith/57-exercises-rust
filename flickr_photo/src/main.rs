#[macro_use]
extern crate rocket;

use rocket_dyn_templates::Template;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
struct Media {
    m: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct Image {
    title: String,
    media: Media,
}

#[derive(Debug, Deserialize, Serialize)]
struct FlickrFeed {
    title: String,
    items: Vec<Image>,
}

#[get("/")]
async fn index() -> Template {
    let mut photos =
        reqwest::get("https://www.flickr.com/services/feeds/photos_public.gne?format=json")
            .await
            .unwrap()
            .text()
            .await
            .unwrap();

    // clean up bad flickr json format
    photos.drain(..photos.find('{').unwrap());
    photos.pop();

    let photos: FlickrFeed = serde_json::from_str(&photos).unwrap();

    Template::render("index", &photos)
}

#[launch]
fn rocket_launch() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
