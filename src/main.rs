/*

wikipedia = helpful
https://en.wikipedia.org/wiki/VIC_cipher

*/
use core::str;
use modular_arithmetic::*;

use std::{collections::HashMap, cmp::Ordering};

const MONTHS: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
const ALPHABET: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];

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

    let d1: &str = &d[0..10];
    let d2: &str = &d[10..20];

    println!("D: {} {}", d1, d2);

    // E
    fn sequentialize (s: &str) -> String {
        let mut alpha_map = HashMap::new();

        for i in 0..10 {
            let char: &str = &(s.chars().nth(i).unwrap()).to_string();

            let alphabet_index = ALPHABET
                .iter()
                .position(|v| v == &char)
                .unwrap_or(0);

            alpha_map.insert(i, alphabet_index);
        }

        let mut hash_vec: Vec<(&usize, &usize)> = alpha_map.iter().collect();
        hash_vec.sort_unstable_by(|a, b| {
            match a.1.cmp(&b.1) {
                Ordering::Equal => { a.0.cmp(&b.0) }
                v => { v }
            }
        });

        let mut output: String = String::from("");

        for i in 0..10 {
            let mut index = hash_vec.iter().position(|&a| a.0 == &i).unwrap() + 1;

            if index >= 10 {
                index = 0;
            }

            output.push_str(&index.to_string());
        }

        output
    }

    let e1: String = sequentialize(d1).to_owned();
    let e2: String = sequentialize(d2).to_owned();

    println!("E: {} {}", e1, e2);

    // F

    fn chain_addition (s: String) -> String {
        let mut output: String = String::from("");

        for i in 0..5 {
            let char1: &str = &(s.chars().nth(i).unwrap()).to_string();
            let digit1: i64 = char1.parse().unwrap();

            let mut iv = i+1;
            if i >= 4 {
                iv = 2;
            }

            let char2: &str = &(s.chars().nth(iv).unwrap()).to_string();
            let digit2: i64 = char2.parse().unwrap();

            let add: u64 = mod_sub(digit1, digit2, 10);

            output.push_str(&add.to_string())
        }

        output
    }

    let mut f1 = c.to_string();
    f1.push_str(
        &chain_addition(c.to_string())
    );

    let f2: String = String::from("1234567890");

    println!("F: {} {}", f1, f2);

    // G - mod10 e.1 + f.1 

    let mut c_str: String = String::from("");

    for i in 0..10 {
        let a_char: char = e1.chars().nth(i).unwrap();
        let b_char: char = f1.chars().nth(i).unwrap();

        const RADIX: u32 = 10;

        let a_int: i64 = a_char.to_digit(RADIX).unwrap().into();
        let b_int: i64 = b_char.to_digit(RADIX).unwrap().into();

        let sub: u64 = mod_sub(a_int, b_int, 10);

        c_str.push_str(&sub.to_string());
    }

    let g: String = c_str.to_string();

    println!("G: {}", g);

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