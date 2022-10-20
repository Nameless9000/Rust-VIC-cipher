/*

wikipedia = helpful
https://en.wikipedia.org/wiki/VIC_cipher

*/
use core::str;
use modular_arithmetic::*;

const MONTHS: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];

fn truncate(s: &str, max_chars: usize) -> &str {
    match s.char_indices().nth(max_chars) {
        None => s,
        Some((idx, _)) => &s[..idx],
    }
}

pub fn generate_key(personal_number: i64, date: &str, phrase: &str, keygroup: i64) -> i64 {
    println!("Personal Number: {personal_number}");
    println!("Date: {date}");
    println!("Phrase: {phrase}");
    println!("Keygroup: {keygroup}");

    // A = keygroup
    let a: i64 = keygroup;
    println!("A: {a}");

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
    let b: i64 = truncate(binding, 5).parse().unwrap();
    println!("B: {b}");

    // C = Subtract B from A by modular arithmetic
    
    let a_str: String = a.to_string();
    let b_str: String = b.to_string();

    let mut c_str: String = String::from("");

    for i in 0..5 {
        let a_char: char = a_str.chars().nth(i).unwrap();
        let b_char: char = b_str.chars().nth(i).unwrap();

        const RADIX: u32 = 10;

        let a_int: i64 = a_char.to_digit(RADIX).unwrap().into();
        let b_int: i64 = b_char.to_digit(RADIX).unwrap().into();

        let sub: u64 = mod_sub(a_int, -b_int, 10);

        c_str.push_str(&sub.to_string());
    }

    let c: i64 = c_str.to_string().parse().unwrap();
    println!("C: {c}");

    // D = Write out the first 20 letters from the secret Phrase
    let phrase_no_space: String = phrase.to_ascii_uppercase().replace(" ", "");
    let d: &str = truncate(&phrase_no_space, 20);
    println!("D: {d}");

    return 0;
}

fn main() {
    let personal_number: i64 = 6;
    let date: &str = "13 Sep 1959";
    let phrase: &str = "Twas the night before Christmas";
    let keygroup: i64 = 72401;

    let key: i64 = generate_key(personal_number, date, phrase, keygroup);

    println!("Key: {key}");
}