use std::io::{self, Write};

pub fn echo(args: &[&str]) -> () {
    if args.len() == 0 {
        println!("expected 1 or more argument; found 0");
        return;
    }
    for &arg in args {
        println!("{}", arg.trim());
        io::stdout().flush().unwrap();
    }
}
