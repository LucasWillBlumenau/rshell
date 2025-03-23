use super::output::Output;

pub fn echo(args: &[&str]) -> Output {
    if args.len() == 0 {
        return Output::err(format!("expected 1 or more argument; found 0"));
    }
    
    let last_index = args.len() - 1;
    let mut buffer = String::new();
    for i in 0..last_index {
        buffer.push_str(&format!("{} ", &args[i]));
    }
    buffer.push_str(args[last_index]);
    return Output::ok(format!("{buffer}"));
}
