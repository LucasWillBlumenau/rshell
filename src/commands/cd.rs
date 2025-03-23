use std::{env::VarError, path::Path};
use super::output::Output;

pub fn cd(args: &[&str]) -> Output {
   let args_length = args.len();
    if args_length != 1 {
        return Output::err(format!("Expected 1 arg; {} found", args_length));
    }

    let dir = expand_user_path(args[0]);
    match dir {
        Ok(dir) => {
            let path = Path::new(&dir);
            if !path.is_dir() {
                return Output::ok(format!("cd: {dir}: No such file or directory"));
            }
        
            let result = std::env::set_current_dir(path);
            if let Err(err) = result {
                return Output::err(format!("{err}"));
            }
            return Output::ok(String::new());
        }
        Err(err) => {
            return Output::err(err.to_string());
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
