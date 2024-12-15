use codecrafters_shell::run_shell;

pub mod commands;

fn main() {

    let path_var = std::env::var("PATH").unwrap();
    run_shell(path_var); 

}   

