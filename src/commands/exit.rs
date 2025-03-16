pub fn exit(args: &str) {
    let code = args.trim();
    let result = code.parse::<i32>();

    match result {
        Ok(code) => {
            std::process::exit(code);
        }
        Err(_) => {
            println!("invalid argument for exit command exit: {}", code)
        }
    }
}