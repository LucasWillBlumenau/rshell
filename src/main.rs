#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, path::Path, process::exit};

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
        io::stdout().flush().unwrap();
    });

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        input = input.trim().to_owned();

        let (command, args) = split(&input, ' ');

        if command == "type" {
            let args = args.trim();
            if commands.contains_key(args) || args == "type" {
                println!("{} is a shell builtin", args);
                continue;
            }
            
            let path = search_file_in_path_envar(args);

            match path {
                Some(path) => {
                    println!("{} is {}", args, path);
                }
                None => {
                    println!("{}: not found", args);
                }
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

fn search_file_in_path_envar(filename: &str) -> Option<String> {

    let path = std::env::var("PATH");
    match path {
        Ok(path) => {
            let paths= path.split(':');
            for path in paths {
                let path = path.to_owned() + "/" + filename;
                if Path::exists(Path::new(&path)) {
                    return Some(path);
                }
            }
            None
        },
        Err(_) => None
    }
}