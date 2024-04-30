use std::io::{self, Write};
use rand::{distributions::Alphanumeric, Rng}; // 0.8

fn main() {
    println!("Password Generator");

    let mut size = String::new();
    // let s = "abcdefghijklmnopqrstuvxzkwyABCDEFGHIJKLMNOPQRSTUVXZKWY1234567890";

    print!("Enter password length: ");
    io::stdout()
        .flush()
        .unwrap();

    io::stdin()
            .read_line(&mut size)
            .expect("Failed to read line");

    let size: usize = size
    .trim()
    .parse()
    .expect("Size entered was not a number");

    let password: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(size)
        .map(char::from)
        .collect();

    println!("Your password is: {} ", password);
}