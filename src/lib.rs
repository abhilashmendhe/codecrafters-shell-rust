use std::path::Path;


pub fn search_directory(allpaths: &[&str], command: String) -> String {
    
    for p in allpaths {
        let fullpath = format!("{}/{}",p,command);
        if Path::new(&fullpath).is_file() {
            return fullpath;
        }
    }
    "".to_string()
}