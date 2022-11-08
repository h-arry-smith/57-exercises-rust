use std::io;
use std::io::Write;

fn main() {
    println!("Enter two strings and I'll tell you if they are anagrams:");
    let this = get_string("Enter the first string: ");
    let that = get_string("Enter the second string: ");

    if anagram_checker(&this, &that) {
        println!("`{}` and `{}` are anagrams", this, that);
    } else {
        println!("`{}` and `{}` are not anagrams", this, that);
    }
}

fn get_string(question_text: &str) -> String {
    print!("{question_text}");
    io::stdout().flush().unwrap();

    let mut str = String::new();

    io::stdin()
        .read_line(&mut str)
        .expect("Failed to read line.");

    str.trim().to_string()
}

fn anagram_checker(this: &str, that: &str) -> bool {
    if this.len() != that.len() {
        return false;
    }

    let mut this_sorted_chars = this.chars().collect::<Vec<char>>();
    let mut that_sorted_chars = that.chars().collect::<Vec<char>>();

    this_sorted_chars.sort();
    that_sorted_chars.sort();

    this_sorted_chars == that_sorted_chars
}
