use std::io;

fn main() {
    println!("Enter a string:");
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    if is_palindrome(&input) {
        println!("The given string is a palindrome.");
    } else {
        println!("The given string is not a palindrome.");
    }
}

fn is_palindrome(input: &str) -> bool {
    let cleaned_input: String = input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .flat_map(char::to_lowercase)
        .collect();

    cleaned_input == cleaned_input.chars().rev().collect::<String>()
}