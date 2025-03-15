#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, process::exit};

fn main() {
    let stdin = io::stdin();
    let mut commands: HashMap<&str, fn(&str) -> ()> = HashMap::new();

    commands.insert("exit", |args: &str| {
        let code = args.trim();
        let result = code.parse::<i32>();

        match result {
            Ok(code) => {
                exit(code);
            }
            Err(_) => {
                println!("invalid argument for exit command exit: {}", code)
            }
        }
    });

    commands.insert("echo", |args: &str| {
        println!("{}", args);
    });




    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        let (command, args) = split(&input, ' ');

        if command == "type" {
            let args = args.trim();
            if commands.contains_key(args) {
                println!("{} is a shell builtin", args)
            } else {
                println!("{}: not found", args)
            }
            continue;
        }


        let func = commands.get(command);
        match func {
            Some(func) => {
                (func)(args)
            }
            None => {
                println!("{}: command not found", command)
            }
        }
        
    }

}


fn split(text: &str, sep: char) -> (&str, &str) {
    let mut index = 0;
    let text_length = text.len();
    for c in text.chars() {
        if c == sep {
            return (&text[0..index], &text[index + 1..text_length])
        }
        index += 1;
    }

    return (&text, "")
}