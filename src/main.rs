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
        
        let trim_input = input.trim();
        if trim_input == "exit 0" {
            break;
        }

        if trim_input.starts_with("echo") {
            println!("{}",&trim_input[5..]);
        } else if trim_input.starts_with("type") {
            let builtin = ["echo", "exit", "type"];
            let mut t_input_spl = trim_input.split(" ");
            let _ = t_input_spl.next().unwrap();
            let command_str = t_input_spl.next().unwrap();

            if builtin.contains(&command_str) {
                println!("{} is a shell builtin",command_str);
            } else {
                println!("{} not found",command_str);
            }
        } else {
            println!("{}: command not found", trim_input);
        }
    }
}
