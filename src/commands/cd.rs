use std::{env::VarError, path::Path};

pub fn cd(args: &str) -> () {
    let args: Vec<&str> = args.trim()
                              .split(' ')
                              .collect();

    let args_length = args.len();
    if args_length != 1 {
        println!("Expected 1 arg; {} found", args_length);
        return;
    }

    let dir = expand_user_path(args[0]);
    match dir {
        Ok(dir) => {
            let path = Path::new(&dir);
            if !path.is_dir() {
                println!("cd: {}: No such file or directory", dir);
                return;
            }
        
            let result = std::env::set_current_dir(path);
            if let Err(err) = result {
                println!("{}", err);
            }
        }
        Err(err) => {
            println!("{}", err);
        }
    } 
 
}


fn expand_user_path(path: &str) -> Result<String, VarError> {
    if !path.starts_with("~") {
        return Ok(path.to_owned());
    }

    let home_dir = std::env::var("HOME");
    match home_dir {
        Ok(home_dir) => Ok(path.replacen("~", &home_dir, 1)),
        Err(err) => Err(err)
    }
}