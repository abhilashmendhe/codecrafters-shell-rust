use codecrafters_shell::run_shell;

// use commands::echo::echo_cmd;

// use codecrafters_shell::commands::echo::echo_cmd;

pub mod commands;

fn main() {

    let path_var = std::env::var("PATH").unwrap();
    run_shell(path_var); 

    // let s = "echo \"           asdf123      \"   asdf 12312312312   123";

    // let s = "echo \"fsdfwer\\\"\"";
    // let s = "echo 'shell\\\\\\ntest'";
    // let s = "echo \"test\\"insidequotes\"example\\"";
    // let s = "echo \"test\\\"insidequotes\"example\\\"";
    // println!("{s}");
    // echo_cmd(s);
}   

