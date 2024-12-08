use std::{fs::read, path::Path};


pub fn search_directory(allpaths: &[&str], command: String) -> String {
    
    for p in allpaths {
        let fullpath = format!("{}/{}",p,command);
        if Path::new(&fullpath).is_file() {
            return fullpath;
        }
    }
    "".to_string()
}

pub fn read_file(path: String) -> Vec<u8> {
    if let Ok(content) = read(path) {
        content
    } else {
        Vec::new()
    }
}