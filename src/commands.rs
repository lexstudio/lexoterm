
use std::fs::{self, File};

use std::io;
use std::net::TcpStream;
use std::path::Path;
use std::process;

pub fn execute_command(input: &str) {
    match input {
        "exit" => {
            println!("Exiting...");
            process::exit(0);
        },
        "hello" => println!("Hello there!"),
        "help" => show_help(),
        "ls" => list_files(),
        cmd if cmd.starts_with("touch") => create_file(cmd),
        cmd if cmd.starts_with("rm") => remove_file(cmd),
        cmd if cmd.starts_with("ip") => show_ip(),
        cmd if cmd.starts_with("rn") => rename_file(cmd),
        cmd if cmd.starts_with("mk") => make_directory(cmd),
        _ => println!("Unknown command: '{}'", input),
    }
}

// Keep the rest of your functions here

// Note: Ensure all other warnings (like unreachable patterns) are resolved by reordering the match arms as shown.

// Existing functions: show_help, list_files, create_file
fn list_files() {
    let current_dir = std::env::current_dir().unwrap();
    println!("Listing files in {:?}", current_dir);
    let entries = fs::read_dir(current_dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        println!("{:?}", path.file_name().unwrap());
    }
}
fn show_help() {
    println!("Available commands:");
    println!("hello - Say hello");
    println!("exit - Exit the terminal");
    println!("help - Show this help message");
    println!("ls - List files in the current directory");
    println!("touch <filename> - Create a new file with the given name");
}
fn create_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: touch <filename>");
        return;
    }

    let file_name = parts[1];
    match File::create(Path::new(file_name)) {
        Ok(_) => println!("{} created successfully.", file_name),
        Err(e) => println!("Error creating file {}: {}", file_name, e),
    }
}
fn remove_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: rm <filename>");
        return;
    }

    let file_name = parts[1];
    match fs::remove_file(file_name) {
        Ok(_) => println!("{} removed successfully.", file_name),
        Err(e) => println!("Error removing file {}: {}", file_name, e),
    }
}

fn show_ip() {
    match TcpStream::connect("google.com:80") {
        Ok(stream) => {
            let local_addr = stream.local_addr().unwrap();
            println!("Local IP address: {}", local_addr.ip());
        }
        Err(e) => println!("Unable to connect: {}", e),
    }
}

fn rename_file(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 3 {
        println!("Usage: rn <oldname> <newname>");
        return;
    }

    let old_name = parts[1];
    let new_name = parts[2];
    match fs::rename(old_name, new_name) {
        Ok(_) => println!("{} renamed to {}.", old_name, new_name),
        Err(e) => println!("Error renaming file: {}", e),
    }
}

fn make_directory(command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.len() < 2 {
        println!("Usage: mk <directory>");
        return;
    }

    let dir_name = parts[1];
    match fs::create_dir(dir_name) {
        Ok(_) => println!("Directory {} created successfully.", dir_name),
        Err(e) => println!("Error creating directory {}: {}", dir_name, e),
    }
}
