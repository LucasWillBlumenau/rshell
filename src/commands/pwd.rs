use super::output::Output;

pub fn pwd(args: &[&str]) -> Output {
    if args.len() != 0 {
        return Output::err(format!("pwd: expected 0 arguments"));
    }

    let current_working_directory = std::env::current_dir();
    match current_working_directory {
        Ok(current_working_directory) => {
            if let Some(path) = current_working_directory.to_str() {
                Output::ok(format!("{}", path))
            } else {
                Output::ok(format!(""))
            }
        }
        Err(err) => Output::err(format!("{}", err.to_string()))
    }
}
