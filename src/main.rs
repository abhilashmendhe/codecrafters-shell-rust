#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    // Uncomment this block to pass the first stage

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        // Wait for user input
        let stdin = io::stdin();
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();
        
        let trim_inp = input.trim();
        if trim_inp == "exit 0" {
            break;
        }

        if trim_inp.starts_with("echo") {
            println!("{}",&trim_inp[5..]);
        } else {
            println!("{}: command not found", trim_inp);
        }
    }
}
