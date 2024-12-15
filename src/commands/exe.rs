use crate::commands::cat::read_file;

pub fn exe_cmd(trim_input: &str) {

    let mut next_str = trim_input;


    if next_str.starts_with('\'') {

        while let Some(ind) = next_str.chars().position(|x| x=='\'') {
            // println!("{}",ind);
            next_str = &next_str[(ind + 1)..];
        }

    } else if next_str.starts_with('"') {
        while let Some(ind) = next_str.chars().position(|x| x=='"') {
            // println!("{}",ind);
            next_str = &next_str[(ind + 1)..];
        }
    } else {

    }
    
    next_str = next_str.trim_ascii_start();
    // println!("{}", next_str);
    
    let mut f = true;
    let mut s = String::new();
    let mut all_file_content = String::new();
    if next_str.starts_with('\'') {
        
        for ch in next_str.chars().collect::<Vec<char>>() {
            if ch == '\'' && f {
                f = false;
                continue;
            }
            if ch == '\''  && !f {
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
    } else if next_str.starts_with('\"') {
        for ch in next_str.chars().collect::<Vec<char>>() {
            if ch == '\"' && f {
                f = false;
                continue;
            }
            if ch == '\"' && !f {
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
    } else {
        let content = read_file(next_str.to_string());
        let content = String::from_utf8_lossy(&content);
        all_file_content.push_str(&content);
    }
    println!("{}", all_file_content.trim());
}