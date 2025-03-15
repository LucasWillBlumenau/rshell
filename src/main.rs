#[allow(unused_imports)]
use std::io::{self, Write};
use std::process::exit;

fn main() {
    let stdin = io::stdin();

    loop {
        print!("$ ");
        io::stdout().flush().unwrap();
        
        let mut input = String::new();
        stdin.read_line(&mut input).unwrap();

        if input.starts_with("exit ") {
            
            let code = get_string_content_after_text(&input, "exit ").trim();
            let result = code.parse::<i32>();

            match result {
                Ok(code) => {
                    exit(code);
                }
                Err(_) => {
                    println!("invalid argument for exit command exit: {}", code)
                }
            }
        } else if input.starts_with("echo ") {
            let message = get_string_content_after_text(&input, "echo ").trim();
            println!("{}", message)
        } else {
            println!("{}: command not found", input.trim())
        }
        
    }

}

fn get_string_content_after_text<'a>(text: &'a str, subtext: &str) -> &'a str {

    let subtext_length = subtext.len();

    for i in 0..text.len() {

        if &text[i..subtext_length + i] == subtext {
            return &text[i + subtext_length..]
        }

    }

    return ""

}