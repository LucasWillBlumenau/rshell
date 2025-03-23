use super::output::Output;

pub fn exit(args: &[&str]) -> Output {
    let args_length = args.len();
    if args_length != 1 {
        return Output::err(format!("expected 1 argument; found {args_length}"));
    }
    let args = args[0];
    let code = args.trim();
    let result = code.parse::<i32>();

    match result {
        Ok(code) => std::process::exit(code),
        Err(_) => Output::ok(format!("invalid argument for exit command exit: {code}"))
    }
}
