use anyhow::Error;

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub redirect_to: Option<String>
}

impl Command {

    pub fn from_cli(cli: &str) -> Result<Self, Error> {
        let redirection_index = Command::find_index(cli, [">", "1>"].as_ref());

        let (cli, file) = if let Some(redirection_index) = redirection_index {
            let (cli, file) = cli.split_at(redirection_index);
            let file = if file.starts_with("1>") { &file[2..] } else { &file[1..] };
            (cli, Some(file.trim().to_owned()))
        } else {
            (cli, None)
        };
        
        let cli = Command::split_args(cli)?;

        if cli.len() == 0 {
            return Err(Error::msg(""));
        }      


        let (command, args) = cli.split_at(1);
        let command = command[0].to_owned();

        Ok(Command {
            name: command,
            args: args.to_owned(),
            redirect_to: file,
        })
    }

    fn find_index<'a>(text: &'a str, slices: &'a [&str]) -> Option<usize> {        
        let min_index = slices.iter()
            .copied()
            .map(|slice| text.find(slice))
            .filter(|index| index.is_some())
            .map(|index| index.unwrap())
            .min();

        min_index
    }


    pub fn split_args(text: &str) -> Result<Vec<String>, Error> {

        let mut text = text.trim_end()
            .trim_start();
    
        let mut args: Vec<String> = vec![];
    
        while text.len() > 0 {
            let index: usize;
            let arg: String;
            if text.starts_with('\'') {
                (index, arg) = Command::search_until_single_quote(&text)?;
            } else if text.starts_with('"') {
                (index, arg) = Command::search_until_double_quote(&text)?;
            } else {
                (index, arg) = Command::search_until_whitespace(&text);
            }
    
            args.push(arg);
            text = &text[index..text.len()].trim_start();
        }
    
        Ok(args)
    }
    
    fn search_until_single_quote(text: &str) -> Result<(usize, String), Error> {
        let mut buffer = String::new();
        let mut dropped_characters_count = 0;
        let mut text = text;
    
        if !text.starts_with('\'') {
            return Err(Error::msg("text not quoted with '"));
        }
    
        text = &text[1..text.len()];
        dropped_characters_count += 1;
    
        let sequences = ["''", "'"];
    
        while let Some((index, seq)) = Command::find_sequences_index(text, &sequences) {
            buffer.push_str(&text[0..index]);
    
            let index = index + seq.len();
            dropped_characters_count += index;
    
            text = &text[index..text.len()];
    
            if seq == "'" {
                return Ok((dropped_characters_count, buffer));
            }
    
        }
        Err(Error::msg("' not closed"))
    }
    
    
    fn search_until_double_quote(text: &str) -> Result<(usize, String), Error> {
        let mut buffer = String::new();
        let mut dropped_characters_count = 0;
        let mut text = text;
    
        if !text.starts_with('"') {
            return Err(Error::msg("text not quoted with \""));
        }
    
        text = &text[1..text.len()];
        dropped_characters_count += 1;
    
        let sequences = ["\\\\", "\"\"", "\" ", "\\\"", "\""];
    
        while let Some((index, seq)) = Command::find_sequences_index(text, &sequences) {
    
            buffer.push_str(&text[0..index]);
    
            let index = index + seq.len();
            dropped_characters_count += index;
    
            text = &text[index..text.len()];
    
            if seq == "\" " {
                return Ok((dropped_characters_count, buffer));
            } else if seq == "\\\\" {
                buffer.push('\\');
            } else if seq == "\\\"" {
                buffer.push('"');
            }
    
        }
    
        let index = text.find('"');
    
        match index {
            Some(index) => {
                buffer.push_str(&text[0..index]);
                dropped_characters_count += index;
    
                Ok((dropped_characters_count, buffer))
            }
            None => Ok((dropped_characters_count, buffer))
        }
        
    }
    
    
    fn search_until_whitespace(text: &str) -> (usize, String) {
        let mut buffer = String::new();
        let mut text = text;
        let mut dropped_characters_count = 0;
    
        let sequences = ["\\ ", "'\\'", "\\'", "\\", " "];
    
        while let Some((index, seq)) = Command::find_sequences_index(text, &sequences) {
            
            buffer.push_str(&text[0..index]);
    
            let index = index + seq.len();
            
            text = &text[index..text.len()];
            dropped_characters_count += index;
    
            if seq == " " {
                return (dropped_characters_count, buffer);
            } else if seq == "\\ " {
                buffer.push(' ');
            } else if seq == "\\'\\" {
                buffer.push('\'');
            } else if seq == "\\\"" {
                buffer.push('"');
            } else if seq == "\\'" {
                buffer.push('\'');
            }
    
        }
    
        buffer.push_str(&text);
        dropped_characters_count += text.len();
    
        return (dropped_characters_count, buffer);
    }
    
    
    fn find_sequences_index<'a>(text: &str, sequences: &[&'a str]) -> Option<(usize, &'a str)> {
        
        let mut index_and_seq: Option<(usize, &str)> = None;
        let mut min_index = text.len();
    
        for seq in sequences.to_owned() {
            
            if seq.len() > text.len() {
                continue;
            }
    
            let iterations = text.len() - seq.len();
    
            for i in 0..=iterations {
                let text = &text[i..i + seq.len()];
                if text == seq && i < min_index {
                    index_and_seq = Some((i, seq));
                    min_index = i;
                    break;
                }
            }
    
        }
        index_and_seq
    
    }
    

}