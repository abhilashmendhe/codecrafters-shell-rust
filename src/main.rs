#[allow(unused_imports)]
use std::io::{self, Write};
use std::{path::Path, process::Command};

use codecrafters_shell::{read_file, search_directory};

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

            let next_str = &trim_input[5..];
            let len = next_str.len();

            if next_str.starts_with('\'') {
                println!("{}",&next_str[1..(len-1)]);

            } else if next_str.starts_with('\"') {
                println!("{}",&next_str[1..(len-1)]);
            } else {
                let mut str = String::new();
                let mut f = true;
                for ch in &next_str.chars().collect::<Vec<char>>() {
                    if ch.is_alphabetic() {
                        str.push(*ch);
                        f = true;
                    } else {
                        if f {
                            str.push(' ');
                            f = false;
                        }
                    }
                }
                println!("{}",str);
            }


        } else if trim_input.starts_with("cat") {

            let next_str = &trim_input[4..];
            // let tmpstr = "'/tmp/bar/f   18' '/tmp/bar/f   33' '/tmp/bar/f   22'";

            let mut f = true;
            let mut s = String::new();
            let mut all_file_content = String::new();
            for ch in next_str.chars().collect::<Vec<char>>() {
                if ch == '\'' && f {
                    f = false;
                    continue;
                }
                if ch == '\'' && !f {
                    // println!("\"{}\"",s);
                    let content = read_file(s);
                    let content = String::from_utf8_lossy(&content);
                    if content.len() > 0 {
                        all_file_content.push_str(&content);
                    }
                    f = true;
                    s = "".to_string();
                }
                if !f {
                    s.push(ch);
                }
            }
            println!("{}", all_file_content.trim());

        } else if trim_input.starts_with("cd") {
            
            let mut t_input_spl = trim_input.split(" ");
            let _ = t_input_spl.next().unwrap();

            if let Some(path) = t_input_spl.next() {

                if path.starts_with("./") {
                    let mut current = std::env::current_dir().unwrap();
                    current.push(&path[2..]);
                    let _ = std::env::set_current_dir(&current);

                } else if path.starts_with('~') {
                    let home_var = std::env::var("HOME").unwrap();
                    let s_path = Path::new(&home_var);
                    let _ = std::env::set_current_dir(&s_path);

                } else  {
                    let s_path = Path::new(path);
                    // println!("{:?}",s_path);
                    let res = std::env::set_current_dir(&s_path);
                    if res.is_err() {
                        println!("cd: {}: No such file or directory", path);
                    }
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

