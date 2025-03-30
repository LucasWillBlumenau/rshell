mod tools;
mod commands;


use core::str;
#[allow(unused_imports)]
use std::io::{self, Write};
use std::{collections::HashMap, fs::File, os::unix::fs::MetadataExt};

use anyhow::Error;
use commands::{command::Redirection, output::Output};
use tools::{cli::CommandLine, paths::search_file_in_path_envar, string};



fn main() {
    
    let mut commands: HashMap<&str, fn(&[&str]) -> Output> = HashMap::new();
    commands.insert("exit", commands::exit::exit);
    commands.insert("echo", commands::echo::echo);
    commands.insert("pwd", commands::pwd::pwd);
    commands.insert("cd", commands::cd::cd);

    let mut cli = CommandLine::new(&commands);

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let input = cli.read();
        if let Err(err) = input {
            println!("Erro reading input: {err}");
            continue;
        }

        let input = input.unwrap()
            .trim()
            .to_owned();


        let cli = commands::command::Command::from_cli(&input);

        if let Err(err) = cli {
            print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(err.to_string().trim()));
            continue;
        }

        let command = cli.unwrap();
        let out = process_command(&commands, &command.name, &command.args);

        if let Some(out_file) = command.redirect_output_to {
            write_to_file_and_log_erros(out_file, &out.stdout);
            if out.stderr.len() > 0 {
                print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(&out.stderr.trim()));
            }
        } else if let Some(out_file) = command.redirect_error_to {
            write_to_file_and_log_erros(out_file, &out.stderr);
            if out.stdout.len() > 0 {
                print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(&out.stdout.trim()));
            }
        } else {
            print!("{}", string::add_new_line_to_string_if_its_missing_and_its_not_empty(out.message().trim()));
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
    Output {
        stdout: String::from_utf8_lossy(&out.stdout).to_string(),
        stderr: String::from_utf8_lossy(&out.stderr).to_string(),
        is_success: out.status.success(),
    }
}


fn write_to_file_and_log_erros(path: Redirection, content: &str) {
    let result = match &path.redirection_type {
        commands::command::RedirectionType::Write => write_to_file(path.path, content),
        commands::command::RedirectionType::Append => append_to_file(path.path, content),
    };
    
    if let Err(err) = result {
        println!("Error writting to file: {err}");
    }
}

fn write_to_file(path: String, content: &str) -> Result<(), Error> {
    let mut file = File::create(path.trim())?;
    file.write_all(content.as_bytes())?;

    Ok(())
}


fn append_to_file(path: String, content: &str) -> Result<(), Error> {
    let mut file = File::options()
        .append(true)
        .create(true)
        .open(path.trim())?;

    let file_metadata = file.metadata()?;
    let file_already_had_content = file_metadata.size() > 0;

    if file_already_had_content {
        file.write_all("\n".as_bytes())?;
    }
    file.write_all(content.trim().as_bytes())?;
    Ok(())
}