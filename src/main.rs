/*

wikipedia = helpful
https://en.wikipedia.org/wiki/VIC_cipher

*/
use core::str;

const MONTHS: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn generate_key(personal_number: i32, date: &str, phrase: &str, keygroup: i32) -> i32 {
    println!("Personal Number: {personal_number}");
    println!("Date: {date}");
    println!("Phrase: {phrase}");
    println!("Keygroup: {keygroup}");

    // A = keygroup
    let a: i32 = keygroup;

    // parse date
    // format: DD Mon(th) YYYY

    let vec: Vec<&str> = date.splitn(3, &" ").collect();
    let day: &str = vec[0];
    let month: &str = vec[1];
    let year: &str = vec[2];

    // # truncate date to 5 digits

    let months_index = MONTHS
        .iter()
        .position(|v| v.to_ascii_lowercase() == month.to_ascii_lowercase())
        .unwrap_or(0) + 1;

    let binding: &String = &(day.to_owned() + &*months_index.to_string() + year);

    // B = truncated date
    let b: i32 = truncate(binding, 5).parse().unwrap();

    // C = Subtract B from A by modular arithmetic
    //let c: i32 = a - b;

    return 0;
}

fn main() {
    let personal_number: i32 = 6;
    let date: &str = "13 Sep 1959";
    let phrase: &str = "Twas the night before Christmas";
    let keygroup: i32 = 72401;

    let key: i32 = generate_key(personal_number, date, phrase, keygroup);

    println!("Key: {key}");
}