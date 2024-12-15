use std::path::Path;

pub fn cd_cmd(trim_input: &str) {
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
}