mod tools;


use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, process::exit};

use self::tools::string::split;
use self::tools::paths::search_file_in_path_envar;


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
            } else if let Some(path) = search_file_in_path_envar(args) {
                println!("{} is {}", args, path);
            } else {
                println!("{}: not found", args);
            }
            continue;
        }

        if let Some(func) = commands.get(command) {
            (func)(args);
        } else if command.starts_with("/") || command.starts_with("./") || command.starts_with("../") {
            let mut process = std::process::Command::new(&command);
            let out = process.args(args.split(' '))
                                     .output()
                                     .expect(&format!("error executing process {}", &command));
            println!("{}", String::from_utf8_lossy(&out.stdout).trim());
        } else if let Some(path) = search_file_in_path_envar(command) {
            let mut process = std::process::Command::new(&path);
            let out = process.args(args.split(' '))
                                     .output()
                                     .expect(&format!("error executing process {}", &path));
            println!("{}", String::from_utf8_lossy(&out.stdout).trim());
        } else {
            println!("{}: command not found", command);
        }
    }
}
