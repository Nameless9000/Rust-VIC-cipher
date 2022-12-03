/*

wikipedia = helpful
https://en.wikipedia.org/wiki/VIC_cipher

*/
use core::{str};
use modular_arithmetic::*;

use std::{collections::HashMap, cmp::Ordering};

const MONTHS: [&str; 12] = ["Jan","Feb","Mar","Apr","May","Jun","Jul","Aug","Sep","Oct","Nov","Dec"];
const ALPHABET: [&str; 26] = ["A","B","C","D","E","F","G","H","I","J","K","L","M","N","O","P","Q","R","S","T","U","V","W","X","Y","Z"];
const NUMBERS: [&str; 10] = ["1","2","3","4","5","6","7","8","9","0"];

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

        let mut first_ans: i64 = 0;

        for i in 0..s.len() {
            let char1: &str = &(s.chars().nth(i).unwrap()).to_string();
            let digit1: i64 = char1.parse().unwrap();

            let mut iv = i+1;
            if i >= s.len() - 1 {iv = 0;}

            let char2: &str = &(s.chars().nth(iv).unwrap()).to_string();
            let mut digit2: i64 = char2.parse().unwrap();

            if i >= s.len() - 1 {
                digit2 = first_ans;
            }

            let add: u64 = mod_sub(digit1, digit2, 10);

            if i == 0 { first_ans = add.try_into().unwrap(); }

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

    let mut g_str: String = String::from("");

    for i in 0..10 {
        let a_char: char = e1.chars().nth(i).unwrap();
        let b_char: char = f1.chars().nth(i).unwrap();

        const RADIX: u32 = 10;

        let a_int: i64 = a_char.to_digit(RADIX).unwrap().into();
        let b_int: i64 = b_char.to_digit(RADIX).unwrap().into();

        let sub: u64 = mod_sub(a_int, b_int, 10);

        g_str.push_str(&sub.to_string());
    }

    let g: String = g_str.to_string();

    println!("G: {}", g);

    // H - encoding

    fn digit_encode (k: String, a: String, s: String) -> String {
        let mut str: String = String::from("");

        for i in 0..s.len() {
            let char: char = s.chars().nth(i).unwrap();
            let index = a.chars().position(|a| a == char).unwrap();

            let new_char: char = k.chars().nth(index).unwrap();

            str.push_str(&new_char.to_string());
        }

        str
    }

    let h = digit_encode(e2, f2, g);

    println!("H: {}", h);

    // J - seq number

    fn number_sequentialize (s: &str) -> String {
        let mut alpha_map = HashMap::new();

        for i in 0..10 {
            let char: &str = &(s.chars().nth(i).unwrap()).to_string();

            let alphabet_index = NUMBERS
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

    let j: String = number_sequentialize(h.as_str());

    println!("J: {}", j);

    // K,L,M,N,P - Chain Addition 50 digits from h

    let binding = chain_addition(h).to_owned();
    let k: &str = binding.as_str();
    println!("K: {}", k);
    let binding = chain_addition(k.to_string()).to_owned();
    let l: &str = binding.as_str();
    println!("L: {}", l);
    let binding = chain_addition(l.to_string()).to_owned();
    let m: &str = binding.as_str();
    println!("M: {}", m);
    let binding = chain_addition(m.to_string()).to_owned();
    let n: &str = binding.as_str();
    println!("N: {}", n);
    let binding = chain_addition(n.to_string()).to_owned();
    let p: &str = binding.as_str();
    println!("P: {}", p);

    let mut block = String::from("");
    block.push_str(&k);
    block.push_str(&l);
    block.push_str(&m);
    block.push_str(&n);
    block.push_str(&p);

    // Q

    let mut digitx1: i64 = 0;
    let mut digitx2: i64 = 0;

    let mut digit_count: i32 = 0;

    for i in 0..9 {
        let char: String = p.chars().nth(9-i).unwrap().to_string();
        let digit: i64 = char.parse().unwrap();

        if digit % 2 == 1 || digit == 0 {
            if digit_count == 0 {
                digitx1 = digit;
                digit_count = 1;
                continue;
            }
            if digit_count == 1 {
                digitx2 = digit;
                break;
            }
        }
    }

    println!("{}, {}", digitx1, digitx2);

    let permutation_length: usize = (personal_number + digitx1 + digitx2).try_into().unwrap();

    fn columnar_transposition (key: &str, msg: &str) -> String {
        let mut plaintext = String::from(msg);
        let key = String::from(key);

        plaintext = format!("{}{}",key,plaintext);

        // Determine the dimensions of the grid
        let columns = key.len();
        let rows = (plaintext.len() as f64 / columns as f64).ceil() as usize;

        // Create a grid to hold the plaintext
        let mut grid: Vec<Vec<char>> = vec![vec![' '; columns]; rows];

        // Fill the grid with the plaintext
        let mut index = 0;
        for row in 0..rows {
            for col in 0..columns {
                if index < plaintext.len() {
                    grid[row][col] = plaintext.chars().nth(index).unwrap();
                    index += 1;
                }
            }
        }

        // Create a new string to hold the ciphertext
        let mut ciphertext = String::new();

        // Read the ciphertext off of the grid according to the key
        for numb in NUMBERS {
            let mut key_index: usize = 0;

            for col in 0..columns {
                let key_char = key.chars().nth(col).unwrap();
                if format!("{}",key_char) == numb {
                    key_index = key.find(key_char).unwrap();
                    break;
                }
            }

            for row in 1..rows {
                ciphertext.push(grid[row][key_index]);
            }
        }

        // return ciphertext
        ciphertext
    }


    let transposed_block = columnar_transposition(j.as_str(), &block);

    let q = &transposed_block[0..permutation_length];

    println!("Q: {}",q);

    // R

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