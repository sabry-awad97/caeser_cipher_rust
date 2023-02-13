use std::{collections::HashMap, io};

const SYMBOLS: &str = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz1234567890 !?.";

fn build_symbol_map() -> HashMap<char, u32> {
    let mut symbol_map = HashMap::new();
    for (i, symbol) in SYMBOLS.chars().enumerate() {
        symbol_map.insert(symbol, i as u32);
    }
    symbol_map
}

fn main() {
    println!("*** Caesar Cipher ***");
    println!("The Caesar cipher encrypts letters by shifting them over by a");
    println!("key number. For example, a key of 2 means the letter A is");
    println!("encrypted into C, the letter B encrypted into D, and so on.");
    println!();

    let mode = get_mode();
    let key = get_key();
    let message = get_message();
    let translated = translate(&mode, key, &message);

    println!("{}", translated);

    if mode.starts_with("e") {
        hack(&translated);
    }
}

fn get_mode() -> String {
    loop {
        println!("Do you want to (e)ncrypt or (d)ecrypt?");
        let mut mode = String::new();
        io::stdin().read_line(&mut mode).unwrap();
        let mode = mode.trim().to_lowercase();

        if mode.starts_with("e") {
            return "encrypt".to_string();
        } else if mode.starts_with("d") {
            return "decrypt".to_string();
        } else {
            println!("Please enter the letter e or d.");
        }
    }
}

fn get_key() -> u32 {
    loop {
        let max_key = SYMBOLS.len() - 1;
        println!("Please enter the key (0 to {}) to use.", max_key);
        let mut key = String::new();
        io::stdin().read_line(&mut key).unwrap();

        if let Ok(key) = key.trim().parse::<u32>() {
            if key <= max_key as u32 {
                return key;
            }
        }

        println!("Invalid key. Please enter a valid key.");
    }
}

fn get_message() -> String {
    println!("Enter the message to encrypt/decrypt:");
    let mut message = String::new();
    io::stdin().read_line(&mut message).unwrap();
    message.trim().to_uppercase()
}

fn translate(mode: &str, key: u32, message: &str) -> String {
    let symbol_map = build_symbol_map();
    let mut translated = String::new();

    for symbol in message.chars() {
        if symbol_map.contains_key(&symbol) {
            let num = symbol_map.get(&symbol).unwrap();
            let num = match mode {
                "encrypt" => (num + key) % SYMBOLS.len() as u32,
                "decrypt" => (num + SYMBOLS.len() as u32 - key) % SYMBOLS.len() as u32,
                _ => panic!("Invalid mode"),
            };
            let translated_symbol = SYMBOLS.chars().nth(num as usize).unwrap();
            translated.push(translated_symbol);
        } else {
            translated.push(symbol);
        }
    }

    translated
}

fn hack(message: &str) {
    for key in 0..SYMBOLS.len() {
        let decrypted_message = translate("decrypt", key as u32, message);
        println!("Key #{}: {}", key, decrypted_message);
    }
}
