use std::io::{self, Write};

pub fn echo(args: &str) -> () {
    println!("{}", args);
    io::stdout().flush().unwrap();
}