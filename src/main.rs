#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::Path, process::Command};

use codecrafters_shell::search_directory;

fn main() {

    let path_var = std::env::var("PATH").unwrap();
    let allpaths = path_var.split(":").collect::<Vec<_>>();

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

        } else if trim_input.starts_with("cd") {
            
            let mut t_input_spl = trim_input.split(" ");
            let _ = t_input_spl.next().unwrap();

            if let Some(path) = t_input_spl.next() {

                let s_path = Path::new(path);
                let res = std::env::set_current_dir(&s_path);
                if res.is_err() {
                    println!("cd: {}: No such file or directory", path);
                }
    
            } else {

                let home_var = std::env::var("HOME").unwrap();
                let s_path = Path::new(&home_var);
                let _ = std::env::set_current_dir(&s_path);

            }

        } else if trim_input.starts_with("pwd") {
            let current = std::env::current_dir().unwrap();
            println!("{}",current.to_str().unwrap());

        } else if trim_input.starts_with("type") {
            let builtin = ["echo", "exit", "type", "pwd", "cd"];
            let mut t_input_spl = trim_input.split(" ");
            let _ = t_input_spl.next().unwrap();
            let command_str = t_input_spl.next().unwrap();

            if builtin.contains(&command_str) {

                println!("{} is a shell builtin",command_str);
            } else {
                let p_ath = search_directory(allpaths.as_slice(), command_str.to_string());
                if p_ath.len() > 0 {
                    println!("{} is {}", command_str, p_ath);
                } else {
                    println!("{}: not found",command_str);
                }
            }
        } else {
            let mut t_input_spl = trim_input.split(" ");
            let program = t_input_spl.next().unwrap();
            // let p_arg = t_input_spl.next();
            let script = search_directory(allpaths.as_slice(), program.to_string());
            if script.len() > 0 {
            
                if let Some(parg) = t_input_spl.next() {

                    if let Ok(output) = Command::new(script)
                        .arg(parg)
                        .output() {
                            let out = String::from_utf8_lossy(&output.stdout);
                            println!("{}",out.trim());
                        }
                }

            } else {
                println!("{}: command not found", trim_input);
            }
        }
    }
}

