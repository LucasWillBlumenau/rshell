use std::path::Path;

pub fn cd(args: &str) -> () {
    let args: Vec<&str> = args.trim()
                              .split(' ')
                              .collect();

    let args_length = args.len();
    if args_length != 1 {
        println!("Expected 1 arg; {} found", args_length);
        return;
    }

    let dir = args[0];
    let path = Path::new(dir);
    if !path.is_dir() {
        println!("cd: {}: No such file or directory", dir);
        return;
    }

    let result = std::env::set_current_dir(path);
    if let Err(err) = result {
        println!("{}", err);
    }
}