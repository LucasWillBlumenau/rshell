mod tools;
mod commands;


use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;

use tools::{paths::search_file_in_path_envar, string::{split, split_args}};



fn main() {
    let stdin = io::stdin();
    let mut commands: HashMap<&str, fn(&[&str]) -> ()> = HashMap::new();

    commands.insert("exit", commands::exit::exit);
    commands.insert("echo", commands::echo::echo);
    commands.insert("pwd", commands::pwd::pwd);
    commands.insert("cd", commands::cd::cd);

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
            let args = split_args(args);
            match args {
                Ok(args) => {
                    let args: Vec<&str> = args.iter().map(|x| &**x).collect();
                    (func)(&args);
                },
                Err(err) => println!("{}", err)
            }
            
        } else if 
            command.starts_with("/") ||
            command.starts_with("./") ||
            command.starts_with("../") ||
            search_file_in_path_envar(command).is_some()
        {
            let mut process = std::process::Command::new(&command);
            let args = split_args(args);
            match args {
                Ok(args) => {
                    let out = process.args(args)
                                     .output()
                                     .expect(&format!("error executing process {}", &command));
                    println!("{}", String::from_utf8_lossy(&out.stdout).trim());
                },
                Err(err) => println!("{}", err)
            };
            
        } else {
            println!("{}: command not found", command);
        }
    }
}
