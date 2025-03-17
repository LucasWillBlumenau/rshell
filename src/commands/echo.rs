use std::io::{self, Write};

pub fn echo(args: &[&str]) -> () {
    if args.len() == 0 {
        println!("expected 1 or more argument; found 0");
        return;
    }
    
    let last_index = args.len() - 1;
    let mut buffer = String::new();
    for i in 0..last_index {
        buffer.push_str(&format!("{} ", &args[i]));
    }
    buffer.push_str(args[last_index]);
    println!("{}", buffer);
    io::stdout().flush().unwrap();
}
