// src/main.rs

mod commands; // This line includes the commands module you've just created.

use std::io::{self, Write};
use commands::execute_command; // Import the function to execute commands.

fn main() {
    loop {
        print!("my_terminal> ");
        io::stdout().flush().unwrap(); // Ensure the prompt appears immediately.

        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let input = input.trim(); // Trim newline characters.
                execute_command(input); // Execute the command using the function from commands.rs
            },
            Err(error) => println!("Error reading line: {}", error),
        }
    }
}