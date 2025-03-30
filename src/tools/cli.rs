use std::io::{self, Write};
use console::{Key, Term};

const BEEP: u8 = 7;


pub struct CommandLine<'a> {
    commands: &'a [&'a str]
}

impl<'a> CommandLine<'a> {

    pub fn new(commands: &'a [&'a str]) -> Self {
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
                    let old_length = buffer.len();

                    term.clear_chars(buffer.len())?;
                    buffer = self.complete_word(buffer);
                    term.write(buffer.as_bytes())?;

                    if old_length == buffer.len() {
                        term.write(&[BEEP])?;
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

    fn complete_word(&self, buffer: String) -> String {

        let index = buffer.find(' ').unwrap_or(buffer.len());
        let (command, args) = buffer.split_at(index);

        let possible_commands: Vec<&str> = self.commands.iter()
            .copied()
            .filter(|key| key.starts_with(command))
            .collect();

        if possible_commands.len() == 1 {
            let command = possible_commands.first().unwrap();
            let mut buffer = String::from(*command);
            if args.len() == 0 {
                buffer.push(' ');
            } else {
                buffer.push_str(args);
            }
            buffer
            
        } else {
            buffer
        }

    }
}