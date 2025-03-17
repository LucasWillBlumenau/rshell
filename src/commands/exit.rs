pub fn exit(args: &[&str]) {
    let args_length = args.len();
    if args_length != 1 {
        println!("expected 1 argument; found {}", args_length);
        return;
    }
    let args = args[0];
    let code = args.trim();
    let result = code.parse::<i32>();

    match result {
        Ok(code) => std::process::exit(code),
        Err(_) => println!("invalid argument for exit command exit: {}", code)
    };
}
