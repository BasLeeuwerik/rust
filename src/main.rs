use std::io; // Import the standard library's input/output module

fn main() {
    println!("We're learning Rust! Please enter your name:");

    let mut name = String::new();

    io::stdin() // Access stdin, the standard input stream
        .read_line(&mut name) // Read user input into the 'name' variable
        .expect("Failed to read line"); // Handle any potential errors

    println!("Hello, {}!", name.trim()); // Print a greeting, trimming any whitespace from the input
}
