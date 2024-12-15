use std::path::Path;
use std::io::{self, Write};
use std::process::Command;

use commands::cat::cat_cmd;
use commands::cd::cd_cmd;
use commands::echo::echo_cmd;
use commands::type_command::type_cmd;

pub mod commands;

pub fn run_shell(path_var: String) {
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
            echo_cmd(trim_input);

        } else if trim_input.starts_with("cat") {
            cat_cmd(trim_input);

        } else if trim_input.starts_with("cd") {
            cd_cmd(trim_input);

        } else if trim_input.starts_with("pwd") {
            let current = std::env::current_dir().unwrap();
            println!("{}",current.to_str().unwrap());

        } else if trim_input.starts_with("type") {
            type_cmd(trim_input, allpaths.clone());

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



pub fn search_directory(allpaths: &[&str], command: String) -> String {
    
    for p in allpaths {
        let fullpath = format!("{}/{}",p,command);
        if Path::new(&fullpath).is_file() {
            return fullpath;
        }
    }
    "".to_string()
}

