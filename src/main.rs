/*

wikipedia = helpful
https://en.wikipedia.org/wiki/VIC_cipher

*/

pub fn generate_key(personal_number: i32, date: &str, phrase: &str, keygroup: i32) -> String {
    println!("Personal Number: {personal_number}");
    println!("Date: {date}");
    println!("Phrase: {phrase}");
    println!("Keygroup: {keygroup}");

    let out: &str = "lol";
    return out.to_string();
}

fn main() {
    let personal_number: i32 = 6;
    let date: &str = "13 Sept 1959";
    let phrase: &str = "Twas the night before Christmas";
    let keygroup: i32 = 72401;

    let key: String = generate_key(personal_number, date, phrase, keygroup);

    println!("Key: {key}");
}