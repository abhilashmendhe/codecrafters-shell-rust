use std::path::Path;

pub fn type_cmd(trim_input: &str, allpaths: Vec<&str>) {
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
}


fn search_directory(allpaths: &[&str], command: String) -> String {
    
    for p in allpaths {
        let fullpath = format!("{}/{}",p,command);
        if Path::new(&fullpath).is_file() {
            return fullpath;
        }
    }
    "".to_string()
}