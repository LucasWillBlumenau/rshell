pub fn pwd(args: &str) -> () {
    if args.trim() != "" {
        println!("pwd: expected 0 arguments");
        return;
    }

    let current_working_directory = std::env::current_dir();
    match current_working_directory {
        Ok(current_working_directory) => {
            if let Some(path) = current_working_directory.to_str() {
                println!("{}", path);
            } else {
                println!("");
            }
        }
        Err(err) => {
            println!("{}", err.to_string())
        }
    }
}