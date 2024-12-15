use std::fs::read;


pub fn cat_cmd(trim_input: &str) {
    let next_str = &trim_input[4..];
    // let tmpstr = "'/tmp/bar/f   18' '/tmp/bar/f   33' '/tmp/bar/f   22'";

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
    }
    println!("{}", all_file_content.trim());
}

pub fn read_file(path: String) -> Vec<u8> {
    if let Ok(content) = read(path) {
        content
    } else {
        Vec::new()
    }
}