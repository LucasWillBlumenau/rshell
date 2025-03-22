use anyhow::Error;

pub fn split(text: &str, sep: char) -> (&str, &str) {
    let mut index = 0;
    let text_length = text.len();
    for c in text.chars() {
        if c == sep {
            return (&text[0..index], &text[index + 1..text_length])
        }
        index += 1;
    }

    return (&text, "")
}

pub fn split_args(text: &str) -> Result<Vec<String>, Error> {

    let mut text = text.trim_end()
        .trim_start();

    let mut args: Vec<String> = vec![];

    while text.len() > 0 {
        let index: usize;
        let arg: String;
        if text.starts_with('\'') {
            (index, arg) = search_until_close_quote(&text, '\'')?;
        } else if text.starts_with('"') {
            (index, arg) = search_until_close_quote(&text, '"')?;
        } else {
            (index, arg) = search_until_whitespace(&text);
        }

        args.push(arg);
        text = &text[index..text.len()].trim_start();
    }

    Ok(args)
}


fn search_until_close_quote(text: &str, quote: char) -> Result<(usize, String), Error> {
    let mut close_buffer = false;
    let mut buffer = String::new();

    for (i, c) in text.chars().enumerate() {
        if i == 0 {
            continue;
        }
        
        if close_buffer {
            if c != quote {
                return Ok((i + 1, buffer));
            }
            close_buffer = false;
        }
        else if c != quote {
            buffer.push(c);
        } else {
            close_buffer = true;
        }
    }

    if close_buffer {
        Ok((text.len(), buffer))
    } else {
        Err(Error::msg(format!("{} not closed", quote)))
    }

}


fn search_until_whitespace(text: &str) -> (usize, String) {
    let mut buffer = String::new();
    let mut text = text;
    let mut dropped_caracters_count = 0;

    let sequences = Box::new(["\\ ", "'\\'", "\\\"", "\\'", "\\", " "]);

    while let Some((index, seq)) = find_sequences_index(text, sequences.as_ref()) {
        
        buffer.push_str(&text[0..index]);

        let index = index + seq.len();
        
        text = &text[index..text.len()];
        dropped_caracters_count += index;

        if seq == " " {
            return (dropped_caracters_count, buffer);
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
    dropped_caracters_count += text.len();

    return (dropped_caracters_count, buffer);
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
