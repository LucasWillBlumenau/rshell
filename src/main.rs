mod tools;
mod commands;


use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, fs::File};

use anyhow::Error;
use commands::output::Output;
use tools::{paths::search_file_in_path_envar, string};



fn main() {
    let stdin = io::stdin();
    let mut commands: HashMap<&str, fn(&[&str]) -> Output> = HashMap::new();

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

        let cli = commands::command::Command::from_cli(&input);

        if let Err(err) = cli {
            print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(err.to_string().trim()));
            io::stdout().flush().unwrap();
            continue;
        }

        let command = cli.unwrap();
        let out = process_command(&commands, &command.name, &command.args);

        if let Some(out_file) = command.redirect_to {
            _ = write_to_file(out_file, &out.stdout);
        } else {
            print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(&out.stdout.trim()));
            io::stdout().flush().unwrap();
        }

        if !out.is_success {
            print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(&out.stderr.trim()));
        }
    }
}


fn process_command(
    commands: &HashMap<&str, fn(&[&str]) -> Output>,
    command_name: &str,
    command_args: &[String]
) -> Output {
    if command_name == "type" {

        if command_args.len() != 1 {
            return Output::err(format!("type got {} args; expected 1", command_args.len()));
        }

        let args = command_args[0].trim();
        if commands.contains_key(args) || args == "type" {
            return Output::ok(format!("{} is a shell builtin", args));
        } else if let Some(path) = search_file_in_path_envar(args) {
            return Output::ok(format!("{} is {}", args, path));
        } else {
            return Output::err(format!("{}: not found", args));
        }
    }

    if let Some(func) = commands.get(command_name) {
        let args: Vec<&str> = command_args.iter().map(|x| &**x).collect();
        (func)(&args)
    } else if 
        command_name.starts_with("/") ||
        command_name.starts_with("./") ||
        command_name.starts_with("../") ||
        search_file_in_path_envar(command_name).is_some()
    {
        execute_process(command_name, command_args)
    } else {
        Output::err(format!("{}: command not found", command_name))
    }
}


fn execute_process(command_name: &str, command_args: &[String]) -> Output {
    let mut process = std::process::Command::new(&command_name);
    let out = process.args(command_args)
                        .output()
                        .expect(&format!("error executing process {}", &command_name));
        

    if out.status.success() {
        Output::ok(format!("{}", String::from_utf8_lossy(&out.stdout).trim()))
    } else {
        Output {
            stdout: String::from_utf8_lossy(&out.stdout).to_string(),
            stderr: String::from_utf8_lossy(&out.stderr).to_string(),
            is_success: false,
        }
    }
}


fn write_to_file(path: String, content: &str) -> Result<(), Error> {
    let mut file = File::create(path)?;
    _ = file.write_all(content.as_bytes())?;

    Ok(())
}
