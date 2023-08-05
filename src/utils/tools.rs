use std::{
    io::{self, BufRead},
    process::Command,
};

pub fn clear() {
    let _ = Command::new("clear").status();
}

pub fn pause() {
    println!("Press 'Enter' to continue...");
    let _ = io::stdin().lock().lines().next();
    clear();
}

pub fn read_input(value: bool) -> bool {
    let mut input = String::new();
    let _ = io::stdin().read_line(&mut input);
    input = input.trim().to_lowercase();
    match value {
        true => {
            if input.is_empty() || input == "y" || input == "yes" {
                return true;
            } else if input == "n" || input == "no" {
                return false;
            }
        }
        false => {
            if input.is_empty() || input == "n" || input == "no" {
                return true;
            } else if input == "y" || input == "yes" {
                return false;
            }
        }
    }
    read_input(value)
}
