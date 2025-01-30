use std::process::Command;
use std::sync::Mutex;
use std::sync::Arc;
use std::{io, vec};
fn read_input() -> i32 {
    loop {
        let mut input = String::new();
        
        println!("Please enter an integer:");

        // Read input from standard input
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line");

        // Trim the input and parse it into an i32
        match input.trim().parse::<i32>() {
            Ok(number) => return number, // Return the valid number
            Err(_) => println!("Invalid input. Please enter a valid integer."),
        }
    }
}

pub static game_state: Mutex<i32> = Mutex::new(0);

pub fn increment_counter(input: i32) {
    let mut num = game_state.lock().unwrap();
    *num += input;
}

pub fn user_menu() -> i32{
    println!("Welcome to Backgammon!");
    println!("Please choose a mode you would like to play:");
    println!("1. AI vs AI");
    println!("2. User vs AI");
    println!("3. User vs User");
    return read_input();
}

pub fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Failed to clear the terminal");
}

