use std::{io::{self, Write}, vec};
use console::{Key, Term};

use super::paths::get_executables_available_in_path;

const BEEP: u8 = 7;


pub struct CommandLine<'a> {
    commands: &'a [&'a str],
    buffered_completion: Vec<String>
}


impl<'a> CommandLine<'a> {

    pub fn new(commands: &'a [&'a str]) -> Self {
        CommandLine { commands, buffered_completion: vec![] }
    }

    pub fn read(&mut self) -> Result<String, io::Error> {
        let mut term = Term::stdout();

        let mut buffer = String::new();

        term.show_cursor()?;
        let mut key = term.read_key()?;

        while key != Key::Enter {

            match key {
                Key::Tab => {
                    if self.buffered_completion.len() > 0 {
                        term.write_line("")?;
                        term.write_line(&self.buffered_completion.join("  "))?;
                        term.write("$ ".as_bytes())?;
                        term.write(buffer.as_bytes())?;
                        term.write(&[BEEP])?;
                        self.buffered_completion.clear();
                        continue;
                    }

                    let old_length: usize = buffer.len();
                    buffer = self.complete_command(buffer);
                    if old_length == buffer.len() {
                        term.write(&[BEEP])?;
                    } else {
                        term.clear_chars(old_length)?;
                        term.write(buffer.as_bytes())?;
                    }
                },
                Key::Backspace => {
                    if buffer.len() > 0 {
                        buffer.remove(buffer.len() - 1);
                        term.clear_chars(1)?;
                    }
                    self.buffered_completion.clear();
                }
                Key::Char(key) => {
                    buffer.push(key);
                    term.write(&[key as u8])?;
                    self.buffered_completion.clear();
                },
                _ => (),
            }
            key = term.read_key()?;
        }
        term.write_line("")?;

        Ok(buffer)
    }

    fn complete_command(&mut self, buffer: String) -> String {

        let index = buffer.find(' ').unwrap_or(buffer.len());
        let (command, args) = buffer.split_at(index);

        let possible_commands: Vec<&str> = self.commands.iter()
            .copied()
            .filter(|key| key.starts_with(command))
            .collect();

        if possible_commands.len() == 1 {
            return create_new_command(args, &possible_commands);
        }
        
        let executables = get_executables_available_in_path();
        let mut executables: Vec<&str> = executables
            .iter()
            .map(|cmd| cmd.as_str())
            .filter(|key| key.starts_with(command))
            .collect();
        
        if executables.len() == 0 {
            return buffer;
        }

        if executables.len() == 1{
            return create_new_command(args, &executables);
        }

        executables.sort();

        let mut complete_until = executables.len();
        for index in 0..executables[..2].len() - 1 {
            let next_index = index + 1;

            let left = executables[index];
            let right = executables[next_index];

            if !right.starts_with(left) || next_index > 1 {
                complete_until = next_index;
                break;
            }
        }

        if complete_until == 1 {
            self.buffered_completion = executables.iter().map(|str| str.to_string()).collect();
            buffer
        } else {
            create_new_command(args, &executables[0..complete_until])
        }

        
    }
}

fn create_new_command(args: &str, possible_commands: &[&str]) -> String {
    let command = possible_commands.first().unwrap();
    let mut buffer = String::from(*command);
    if args.len() == 0 && possible_commands.len() == 1 {
        buffer.push(' ');
    } else if args.len() > 0 {
        buffer.push_str(args);
    }
    buffer
}