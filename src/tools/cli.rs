use std::{collections::HashMap, io::{self, Write}};

use console::{Key, Term};

use crate::commands::output::Output;


pub struct CommandLine<'a> {
    commands: &'a HashMap<&'a str, fn(&[&str]) -> Output>
}

impl<'a> CommandLine<'a> {

    pub fn new(commands: &'a HashMap<&'a str, fn(&[&str]) -> Output>) -> Self {
        CommandLine { commands }
    }

    pub fn read(&mut self) -> Result<String, io::Error> {
        let mut term = Term::stdout();
        let mut buffer = String::new();
        let mut key = term.read_key()?;

        term.show_cursor()?;

        while key != Key::Enter {

            match key {
                Key::Tab => {
                    let remaing_to_complete = self.complete_word(&mut buffer);
                    if let Some(remaing_to_complete) = remaing_to_complete {
                        buffer.push_str(remaing_to_complete);
                        term.write(remaing_to_complete.as_bytes())?;
                        term.write(" ".as_bytes())?;
                    }
                },
                Key::Backspace => {
                    if buffer.len() > 0 {
                        buffer.remove(buffer.len() - 1);
                        term.clear_chars(1)?;
                    }
                }
                Key::Char(key) => {
                    buffer.push(key);
                    term.write(format!("{key}").as_bytes())?;
                },
                _ => (),
            }
            key = term.read_key()?;
        }
        term.write_line("")?;
        Ok(buffer)
    }

    fn complete_word(&self, buffer: &mut String) -> Option<&str> {

        let possible_commands: Vec<&str> = self.commands.keys()
            .copied()
            .filter(|key| key.starts_with(buffer.as_str()))
            .collect();

        if possible_commands.len() == 1 {
           let command = possible_commands.first().unwrap();
           Some(&command[buffer.len()..command.len()])
        } else {
            None
        }

    }
}