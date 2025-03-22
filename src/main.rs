mod tools;
mod commands;


use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::collections::HashMap;

use tools::{paths::search_file_in_path_envar, string::split_args};



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

        let cli = split_args(&input);
        if let Err(err) = cli {
            println!("{}", err);
            continue;
        }

        let cli = cli.unwrap();

        if cli.len() == 0 {
            continue;
        }

        let (command, args) = cli.split_at(1);
        let command = command[0].as_str();

        if command == "type" {

            if args.len() != 1 {
                println!("type got {} args; expected 1", args.len());
                continue;
            }

            let args = args[0].trim();
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
            let args: Vec<&str> = args.iter().map(|x| &**x).collect();
            (func)(&args);            
        } else if 
            command.starts_with("/") ||
            command.starts_with("./") ||
            command.starts_with("../") ||
            search_file_in_path_envar(command).is_some()
        {
            let mut process = std::process::Command::new(&command);
            let out = process.args(args)
                                .output()
                                .expect(&format!("error executing process {}", &command));
            println!("{}", String::from_utf8_lossy(&out.stdout).trim());            
        } else {
            println!("{}: command not found", command);
        }
    }
}
