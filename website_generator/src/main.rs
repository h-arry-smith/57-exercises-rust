use std::{
    fs::{self, File},
    io::{self, Write},
    path::Path,
};

fn create_site_folder(site_name: &str) {
    fs::create_dir(site_name).expect("Could not create dir {site_name}");
    println!("Created ./{site_name}");
}

fn create_site_index(site_name: &str, author: &str) {
    let path = Path::new(site_name).join("index.html");
    let mut f = File::create(path).expect("Could not create index at ./{site_name}/index.html");
    let html = format!("<!DOCTYPE html>\n<html>\n\t<head>\n\t\t<title>{site_name}</title>\n\t\t<meta name=\"author\">{author}</meta>\n\t</head>\n</html>");
    f.write(html.as_bytes()).expect("Failed to write.");
    println!("Created ./{site_name}/index.html");
}

fn create_folder(site_name: &str, folder_name: &str) {
    let path = Path::new(site_name).join(folder_name);
    fs::create_dir(path).expect("Could not create dir ./{site_name}");
    println!("Created ./{site_name}/{folder_name}");
}

fn main() {
    let site_name = get_string("Site name: ");
    let author = get_string("Author: ");
    let js_folder = get_bool("Do you want a folder for Javascript? ");
    let css_folder = get_bool("Do you want a folder for CSS? ");

    create_site_folder(&site_name);
    create_site_index(&site_name, &author);
    if js_folder {
        create_folder(&site_name, "js");
    }
    if css_folder {
        create_folder(&site_name, "css");
    }
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

fn get_bool(question_text: &str) -> bool {
    loop {
        let answer = get_string(question_text);

        match answer.as_str() {
            "y" => return true,
            "f" => return false,
            _ => continue,
        }
    }
}
